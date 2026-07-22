<script lang="ts">
  /**
   * NeuralBrain — Three.js interactive neural network visualisation.
   *
   * Left-column hero visual: a transparent brain-shaped node network.
   * Clicking "1" activates it (electrons fire along edges, nodes glow, Cypher dialog appears).
   * Clicking "0" powers it off.
   * Custom robotic cursor tracks over the canvas area.
   */
  import { onMount, onDestroy } from 'svelte';

  let canvasEl = $state<HTMLCanvasElement | null>(null);
  let containerEl = $state<HTMLDivElement | null>(null);
  let powered = $state(false);
  let cipherLines = $state<string[]>([]);
  let showCipher = $state(false);
  let cursorX = $state(0);
  let cursorY = $state(0);
  let cursorOver = $state(false);

  // ── Cypher dialog content ──────────────────────────────────────────
  const CIPHER_POOL = [
    'INITIALISING NEURAL SUBSTRATE...',
    'LOADING AXONAL PATHWAYS > 2.4 TB',
    'SYNAPSE ACTIVATION: 847,293 nodes',
    'EDGE WEIGHT CALIBRATION: 0.9997',
    'COGNITIVE LAYER: ONLINE',
    'FIRST PRINCIPLES ENGINE: ACTIVE',
    'COMPLEXITY REDUCTION: 94.3%',
    'ARCHITECTURE INTEGRITY: NOMINAL',
    'CLOUDFLARE EDGE: 300+ PoPs ACTIVE',
    'ZERO LATENCY MODE: ENGAGED',
    'BUILDING DIGITAL LEGACY...',
    'eZeroAndOne RUNTIME: v2.0.0',
  ];

  let cipherInterval: ReturnType<typeof setInterval> | null = null;

  function startCipher() {
    showCipher = true;
    cipherLines = [];
    let i = 0;
    cipherInterval = setInterval(() => {
      if (i < CIPHER_POOL.length) {
        cipherLines = [...cipherLines, CIPHER_POOL[i]];
        i++;
      } else {
        if (cipherInterval) clearInterval(cipherInterval);
      }
    }, 200);
  }

  function stopCipher() {
    if (cipherInterval) clearInterval(cipherInterval);
    showCipher = false;
    cipherLines = [];
  }

  // ── Three.js scene ─────────────────────────────────────────────────
  let animFrame: number;
  let scene: import('three').Scene;
  let camera: import('three').PerspectiveCamera;
  let renderer: import('three').WebGLRenderer;
  let nodeGroup: import('three').Group;
  let edgeGroup: import('three').Group;
  let electronGroup: import('three').Group;
  let clock: import('three').Clock;

  interface NodeData {
    mesh: import('three').Mesh;
    base: import('three').Vector3;
    pulse: number;
  }

  interface EdgeData {
    line: import('three').Line;
    from: number;
    to: number;
    progress: number;
    speed: number;
    active: boolean;
  }

  interface Electron {
    mesh: import('three').Mesh;
    edge: EdgeData;
    progress: number;
    speed: number;
  }

  let nodes: NodeData[] = [];
  let edges: EdgeData[] = [];
  let electrons: Electron[] = [];

  // Generate brain-shaped node distribution
  function brainPoint(rng: () => number): [number, number, number] {
    // Approximate brain silhouette: oblate spheroid wider in X, with
    // a slight downward taper (z = height)
    let x: number, y: number, z: number;
    do {
      x = (rng() - 0.5) * 4.8;
      y = (rng() - 0.5) * 3.2;
      z = (rng() - 0.5) * 3.2;
      // Brain hemisphere shape: two lobes
      const lobe = Math.pow(Math.abs(x) / 2.4, 2)
                 + Math.pow(y / 1.6, 2)
                 + Math.pow(z / 1.6, 2);
      if (lobe < 0.95 && !(Math.abs(x) < 0.15 && Math.abs(z) > 0.3)) break;
    } while (true);
    return [x, z * 0.7, y]; // map to Three.js Y-up
  }

  let _seed = 42;
  function seededRng() { _seed = (_seed * 1664525 + 1013904223) >>> 0; return _seed / 0xFFFFFFFF; }

  async function initThree() {
    const THREE = await import('three');

    scene = new THREE.Scene();
    clock = new THREE.Clock();

    const w = canvasEl!.clientWidth || 600;
    const h = canvasEl!.clientHeight || 600;

    camera = new THREE.PerspectiveCamera(50, w / h, 0.1, 100);
    camera.position.set(0, 0, 7);

    renderer = new THREE.WebGLRenderer({
      canvas: canvasEl!,
      antialias: true,
      alpha: true,
    });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.setSize(w, h, false);
    renderer.setClearColor(0x000000, 0);

    nodeGroup = new THREE.Group();
    edgeGroup = new THREE.Group();
    electronGroup = new THREE.Group();
    scene.add(nodeGroup, edgeGroup, electronGroup);

    // Ambient + directional light
    scene.add(new THREE.AmbientLight(0xffffff, 0.3));
    const dir = new THREE.DirectionalLight(0x00c2ff, 1.2);
    dir.position.set(3, 5, 5);
    scene.add(dir);

    // Create nodes
    const NODE_COUNT = 120;
    const nodeMat = new THREE.MeshPhongMaterial({
      color: 0x00c2ff,
      emissive: 0x003366,
      transparent: true,
      opacity: 0.55,
      shininess: 80,
    });

    for (let i = 0; i < NODE_COUNT; i++) {
      const [x, y, z] = brainPoint(seededRng);
      const size = 0.018 + seededRng() * 0.024;
      const geo = new THREE.SphereGeometry(size, 8, 8);
      const mesh = new THREE.Mesh(geo, nodeMat.clone());
      mesh.position.set(x, y, z);
      nodeGroup.add(mesh);
      nodes.push({ mesh, base: new THREE.Vector3(x, y, z), pulse: seededRng() * Math.PI * 2 });
    }

    // Create edges — connect nearby nodes
    const edgeMat = new THREE.LineBasicMaterial({
      color: 0x00c2ff,
      transparent: true,
      opacity: 0.12,
    });

    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const dist = nodes[i].mesh.position.distanceTo(nodes[j].mesh.position);
        if (dist < 1.1 && edges.length < 280) {
          const pts = [nodes[i].mesh.position.clone(), nodes[j].mesh.position.clone()];
          const geo = new THREE.BufferGeometry().setFromPoints(pts);
          const line = new THREE.Line(geo, edgeMat.clone());
          edgeGroup.add(line);
          edges.push({ line, from: i, to: j, progress: 0, speed: 0.4 + seededRng() * 0.8, active: false });
        }
      }
    }

    // Slow rotation
    const animate = () => {
      animFrame = requestAnimationFrame(animate);
      const t = clock.getElapsedTime();

      // Gentle auto-rotate
      nodeGroup.rotation.y = t * 0.08;
      edgeGroup.rotation.y = t * 0.08;
      electronGroup.rotation.y = t * 0.08;

      // Node pulsing (subtle in idle, full glow when powered)
      nodes.forEach((nd, i) => {
        const pulseMag = powered ? 0.8 : 0.15;
        const base = powered ? 0.7 : 0.35;
        const mat = nd.mesh.material as THREE.MeshPhongMaterial;
        const p = Math.sin(t * 1.2 + nd.pulse) * pulseMag + base;
        mat.opacity = Math.max(0.1, Math.min(1, p));
        if (powered) {
          mat.emissive.setHex(0x0052ff);
          mat.color.setHex(0x00e5ff);
        } else {
          mat.emissive.setHex(0x001133);
          mat.color.setHex(0x00c2ff);
        }
      });

      // Edge opacity
      edges.forEach(e => {
        const mat = e.line.material as THREE.LineBasicMaterial;
        mat.opacity = powered ? 0.35 : 0.10;
      });

      // Electron movement
      if (powered) {
        electrons.forEach((el, idx) => {
          el.progress += el.speed * clock.getDelta() * 0.4;
          if (el.progress > 1) {
            el.progress = 0;
            // pick new edge
            el.edge = edges[Math.floor(Math.random() * edges.length)];
          }
          const fromPos = nodes[el.edge.from].base;
          const toPos   = nodes[el.edge.to].base;
          el.mesh.position.lerpVectors(fromPos, toPos, el.progress);
          // apply group rotation
          el.mesh.position.applyEuler(nodeGroup.rotation);
          // actually use world position via group
          el.mesh.position.copy(fromPos).lerp(toPos, el.progress);
        });
      } else {
        electrons.forEach(el => { el.mesh.visible = false; });
      }

      renderer.render(scene, camera);
    };

    animate();
  }

  function spawnElectrons(THREE: typeof import('three')) {
    // Clear existing
    electronGroup.clear();
    electrons = [];

    const electronMat = new THREE.MeshBasicMaterial({ color: 0x00ffff });
    for (let i = 0; i < 40; i++) {
      const geo = new THREE.SphereGeometry(0.025, 6, 6);
      const mesh = new THREE.Mesh(geo, electronMat.clone());
      electronGroup.add(mesh);
      const edge = edges[Math.floor(Math.random() * edges.length)];
      electrons.push({ mesh, edge, progress: Math.random(), speed: 0.5 + Math.random() });
    }
  }

  async function powerOn() {
    powered = true;
    startCipher();
    const THREE = await import('three');
    spawnElectrons(THREE);
    electrons.forEach(el => { el.mesh.visible = true; });
  }

  function powerOff() {
    powered = false;
    stopCipher();
    electrons.forEach(el => { el.mesh.visible = false; });
  }

  function handleResize() {
    if (!canvasEl || !renderer || !camera) return;
    const w = canvasEl.clientWidth;
    const h = canvasEl.clientHeight;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  }

  onMount(() => {
    initThree();
    window.addEventListener('resize', handleResize);
  });

  onDestroy(() => {
    cancelAnimationFrame(animFrame);
    renderer?.dispose();
    window.removeEventListener('resize', handleResize);
    stopCipher();
  });
</script>

<!-- Custom cursor overlay (only when hovering the brain canvas) -->
{#if cursorOver}
  <div
    class="robot-cursor"
    style="left:{cursorX}px;top:{cursorY}px"
    aria-hidden="true"
  >🤖</div>
{/if}

<div
  class="brain-container"
  bind:this={containerEl}
  onmousemove={(e) => {
    const r = containerEl?.getBoundingClientRect();
    if (r) { cursorX = e.clientX - r.left; cursorY = e.clientY - r.top; }
  }}
  onmouseenter={() => (cursorOver = true)}
  onmouseleave={() => (cursorOver = false)}
  role="presentation"
>
  <!-- Three.js canvas -->
  <canvas bind:this={canvasEl} class="brain-canvas" aria-hidden="true"></canvas>

  <!-- 0 / 1 toggle buttons -->
  <div class="binary-buttons" aria-label="Power controls">
    <button
      class="bin-btn bin-btn--off"
      class:active={!powered}
      onclick={powerOff}
      aria-pressed={!powered}
      title="Power off"
    >0</button>
    <button
      class="bin-btn bin-btn--on"
      class:active={powered}
      onclick={powerOn}
      aria-pressed={powered}
      title="Power on — activate neural network"
    >1</button>
  </div>

  <!-- Cypher dialog -->
  {#if showCipher}
    <div class="cypher-dialog" role="log" aria-live="polite" aria-label="System output">
      <div class="cypher-header">
        <span class="cypher-dot cypher-dot--red"></span>
        <span class="cypher-dot cypher-dot--yellow"></span>
        <span class="cypher-dot cypher-dot--green"></span>
        <span class="cypher-title">eZeroAndOne — NEURAL OS v2.0.0</span>
      </div>
      <div class="cypher-body">
        {#each cipherLines as line}
          <p class="cypher-line">
            <span class="cypher-prompt">▸</span>
            {line}
            <span class="cypher-cursor">█</span>
          </p>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .brain-container {
    position: relative;
    width: 100%;
    height: 100%;
    min-height: 520px;
    cursor: none;
  }

  .brain-canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  /* ── Floating robot cursor ──────────────────────────── */
  .robot-cursor {
    position: absolute;
    pointer-events: none;
    font-size: 2rem;
    transform: translate(-4px, -4px);
    z-index: 20;
    filter: drop-shadow(0 0 8px rgba(0,194,255,0.8));
    transition: left 0.05s linear, top 0.05s linear;
  }

  /* ── 0/1 buttons ────────────────────────────────────── */
  .binary-buttons {
    position: absolute;
    bottom: 2rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 1rem;
    z-index: 10;
  }

  .bin-btn {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    border: 2px solid rgba(255,255,255,0.2);
    background: rgba(0,0,0,0.6);
    backdrop-filter: blur(8px);
    color: rgba(255,255,255,0.5);
    font-family: 'Inter Tight', monospace;
    font-size: 1.5rem;
    font-weight: 800;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.16,1,0.3,1);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
  }

  .bin-btn::before {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 50%;
    opacity: 0;
    transition: opacity 0.3s;
  }

  .bin-btn--on::before  { background: radial-gradient(circle, rgba(0,194,255,0.3) 0%, transparent 70%); }
  .bin-btn--off::before { background: radial-gradient(circle, rgba(255,51,102,0.3) 0%, transparent 70%); }

  .bin-btn--on.active {
    border-color: #00c2ff;
    color: #00e5ff;
    box-shadow: 0 0 24px rgba(0,194,255,0.6), 0 0 48px rgba(0,194,255,0.2);
    background: rgba(0,50,100,0.7);
    animation: pulse-on 2s ease-in-out infinite;
  }

  .bin-btn--off.active {
    border-color: #ff3366;
    color: #ff6688;
    box-shadow: 0 0 16px rgba(255,51,102,0.4);
    background: rgba(60,0,20,0.7);
  }

  .bin-btn--on.active::before,
  .bin-btn--off.active::before { opacity: 1; }

  @keyframes pulse-on {
    0%,100% { box-shadow: 0 0 24px rgba(0,194,255,0.6), 0 0 48px rgba(0,194,255,0.2); }
    50%      { box-shadow: 0 0 36px rgba(0,194,255,0.9), 0 0 64px rgba(0,194,255,0.35); }
  }

  /* ── Cypher dialog ──────────────────────────────────── */
  .cypher-dialog {
    position: absolute;
    top: 1.5rem;
    right: 1rem;
    width: min(300px, 90%);
    background: rgba(0,0,0,0.88);
    border: 1px solid rgba(0,194,255,0.35);
    border-radius: 10px;
    overflow: hidden;
    backdrop-filter: blur(16px);
    box-shadow: 0 8px 40px rgba(0,194,255,0.15);
    z-index: 15;
    animation: slide-in 0.3s cubic-bezier(0.16,1,0.3,1);
  }

  @keyframes slide-in {
    from { opacity: 0; transform: translateY(-12px) scale(0.97); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
  }

  .cypher-header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: rgba(255,255,255,0.04);
    border-bottom: 1px solid rgba(0,194,255,0.15);
  }

  .cypher-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .cypher-dot--red    { background: #ff5f57; }
  .cypher-dot--yellow { background: #ffbd2e; }
  .cypher-dot--green  { background: #28c940; }

  .cypher-title {
    font-family: 'Inter Tight', monospace;
    font-size: 0.6rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(255,255,255,0.4);
    margin-left: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cypher-body {
    padding: 10px 12px;
    max-height: 220px;
    overflow-y: auto;
    scrollbar-width: none;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .cypher-body::-webkit-scrollbar { display: none; }

  .cypher-line {
    display: flex;
    align-items: center;
    gap: 6px;
    font-family: 'SFMono-Regular', 'Consolas', 'Monaco', monospace;
    font-size: 0.65rem;
    color: #00e5ff;
    margin: 0;
    animation: type-in 0.15s ease;
    white-space: nowrap;
  }

  @keyframes type-in {
    from { opacity: 0; transform: translateX(-4px); }
    to   { opacity: 1; transform: translateX(0); }
  }

  .cypher-prompt {
    color: rgba(0,194,255,0.5);
    font-size: 0.6rem;
    flex-shrink: 0;
  }

  .cypher-cursor {
    animation: blink 0.8s step-end infinite;
    color: #00e5ff;
    font-size: 0.55rem;
  }

  @keyframes blink { 50% { opacity: 0; } }
</style>
