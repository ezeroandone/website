<script lang="ts">
  /**
   * NeuralBrain — Three.js neural network in a human brain silhouette.
   * Client-only. Must be rendered inside {#if browser} on the parent.
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
      if (i < CIPHER_POOL.length) { cipherLines = [...cipherLines, CIPHER_POOL[i]]; i++; }
      else { if (cipherInterval) clearInterval(cipherInterval); }
    }, 200);
  }

  function stopCipher() {
    if (cipherInterval) clearInterval(cipherInterval);
    showCipher = false;
    cipherLines = [];
  }

  // ── Scene state ───────────────────────────────────────────────────
  let animFrame: number;
  let scene: any, camera: any, renderer: any;
  let nodeGroup: any, edgeGroup: any, electronGroup: any, clock: any;

  interface NodeData { mesh: any; base: any; pulse: number; }
  interface EdgeData { line: any; from: number; to: number; active: boolean; }
  interface Electron { mesh: any; edgeIdx: number; progress: number; speed: number; }

  let nodes: NodeData[] = [];
  let edges: EdgeData[] = [];
  let electrons: Electron[] = [];

  // ── Human brain silhouette point cloud ───────────────────────────
  // Defined as a set of parametric regions matching a real brain:
  // - Large left/right hemispheres (top)
  // - Cerebellum (rear lower)
  // - Brain stem (bottom)
  function brainPoints(count: number): [number, number, number][] {
    const pts: [number, number, number][] = [];
    let _s = 12345;
    const rng = () => { _s = (_s * 1664525 + 1013904223) >>> 0; return _s / 0xFFFFFFFF; };

    const inBrain = (x: number, y: number, z: number): boolean => {
      // Main hemispheres: two offset ellipsoids
      const lx = x - 0.5, rx = x + 0.5;
      const leftLobe  = (lx*lx)/(1.8*1.8) + (y*y)/(1.6*1.6) + (z*z)/(1.3*1.3);
      const rightLobe = (rx*rx)/(1.8*1.8) + (y*y)/(1.6*1.6) + (z*z)/(1.3*1.3);
      if (leftLobe < 1.0 || rightLobe < 1.0) {
        // Exclude bottom flat part — brain sits above y = -1
        if (y > -0.9) return true;
      }
      // Cerebellum: smaller ellipsoid rear-lower
      const cy = y + 1.1, cz = z + 1.1;
      const cerebellum = (x*x)/(1.2*1.2) + (cy*cy)/(0.7*0.7) + (cz*cz)/(0.9*0.9);
      if (cerebellum < 1.0) return true;
      // Brain stem: narrow cylinder bottom-center
      const stemDist = Math.sqrt(x*x + (z+0.6)*(z+0.6));
      if (stemDist < 0.35 && y > -2.0 && y < -0.7) return true;
      return false;
    };

    let attempts = 0;
    while (pts.length < count && attempts < count * 30) {
      const x = (rng() - 0.5) * 5.0;
      const y = (rng() - 0.5) * 4.5 + 0.2;
      const z = (rng() - 0.5) * 3.5;
      if (inBrain(x, y, z)) pts.push([x, y, z]);
      attempts++;
    }
    return pts;
  }

  // ── Init Three.js ─────────────────────────────────────────────────
  async function initThree() {
    const THREE = await import('three');

    scene = new THREE.Scene();
    clock = new THREE.Clock();

    const w = canvasEl!.clientWidth  || 560;
    const h = canvasEl!.clientHeight || 560;

    camera = new THREE.PerspectiveCamera(52, w / h, 0.1, 100);
    camera.position.set(0, 0.2, 8);

    renderer = new THREE.WebGLRenderer({ canvas: canvasEl!, antialias: true, alpha: true });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.setSize(w, h, false);
    renderer.setClearColor(0x000000, 0);

    nodeGroup     = new THREE.Group();
    edgeGroup     = new THREE.Group();
    electronGroup = new THREE.Group();
    scene.add(nodeGroup, edgeGroup, electronGroup);

    scene.add(new THREE.AmbientLight(0xffffff, 0.4));
    const dir = new THREE.DirectionalLight(0x00c2ff, 1.4);
    dir.position.set(4, 6, 5);
    scene.add(dir);
    const rimLight = new THREE.PointLight(0x0052ff, 0.8, 12);
    rimLight.position.set(-4, 2, -3);
    scene.add(rimLight);

    // ── Nodes ─────────────────────────────────────────────────────
    const NODE_COUNT = 140;
    const positions = brainPoints(NODE_COUNT);

    positions.forEach(([x, y, z]) => {
      let _s2 = Math.abs(x * 1000 + y * 7777 + z * 3333) | 0;
      const r2 = () => { _s2 = (_s2 * 1664525 + 1013904223) >>> 0; return _s2 / 0xFFFFFFFF; };

      const size = 0.016 + r2() * 0.022;
      const geo  = new THREE.SphereGeometry(size, 7, 7);
      const mat  = new THREE.MeshPhongMaterial({
        color: 0x00c2ff, emissive: 0x001844,
        transparent: true, opacity: 0.5, shininess: 90,
      });
      const mesh = new THREE.Mesh(geo, mat);
      mesh.position.set(x, y, z);
      nodeGroup.add(mesh);
      nodes.push({ mesh, base: new THREE.Vector3(x, y, z), pulse: r2() * Math.PI * 2 });
    });

    // ── Edges ──────────────────────────────────────────────────────
    const edgeMat = new THREE.LineBasicMaterial({ color: 0x0088cc, transparent: true, opacity: 0.10 });

    for (let i = 0; i < nodes.length; i++) {
      let connections = 0;
      for (let j = i + 1; j < nodes.length && connections < 4; j++) {
        const d = nodes[i].mesh.position.distanceTo(nodes[j].mesh.position);
        if (d < 1.0) {
          const geo  = new THREE.BufferGeometry().setFromPoints([
            nodes[i].mesh.position.clone(),
            nodes[j].mesh.position.clone(),
          ]);
          const line = new THREE.Line(geo, edgeMat.clone());
          edgeGroup.add(line);
          edges.push({ line, from: i, to: j, active: false });
          connections++;
        }
      }
    }

    // ── Brain enclosure mesh ───────────────────────────────────────
    // A semi-transparent ellipsoid shell that encloses the neuron network,
    // giving the visual impression of a brain boundary
    const brainShellGeo = new THREE.SphereGeometry(2.3, 48, 48);

    // Displace vertices to create a lobe-like brain outline
    const posAttr = brainShellGeo.attributes.position;
    for (let i = 0; i < posAttr.count; i++) {
      const x = posAttr.getX(i);
      const y = posAttr.getY(i);
      const z = posAttr.getZ(i);
      // Scale to hemisphere proportions (wider X, narrower Z)
      posAttr.setXYZ(i, x * 1.15, y * 1.0, z * 0.85);
    }
    brainShellGeo.computeVertexNormals();

    // Outer glow shell — very transparent cyan wireframe
    const brainWireMat = new THREE.MeshBasicMaterial({
      color: 0x00c2ff,
      wireframe: true,
      transparent: true,
      opacity: 0.06,
    });
    const brainWireMesh = new THREE.Mesh(brainShellGeo, brainWireMat);
    brainWireMesh.position.y = 0.1;
    scene.add(brainWireMesh);

    // Inner surface — frosted glass look
    const brainSurfaceGeo = new THREE.SphereGeometry(2.25, 32, 32);
    const surfacePos = brainSurfaceGeo.attributes.position;
    for (let i = 0; i < surfacePos.count; i++) {
      surfacePos.setXYZ(i,
        surfacePos.getX(i) * 1.15,
        surfacePos.getY(i) * 1.0,
        surfacePos.getZ(i) * 0.85,
      );
    }
    brainSurfaceGeo.computeVertexNormals();

    const brainSurfaceMat = new THREE.MeshPhongMaterial({
      color: 0x003355,
      emissive: 0x000822,
      transparent: true,
      opacity: 0.08,
      side: THREE.BackSide, // render inside-out so it doesn't occlude nodes
      shininess: 60,
    });
    const brainSurfaceMesh = new THREE.Mesh(brainSurfaceGeo, brainSurfaceMat);
    brainSurfaceMesh.position.y = 0.1;
    scene.add(brainSurfaceMesh);

    // Store refs to animate shell with node group
    nodeGroup.userData.wireMesh    = brainWireMesh;
    nodeGroup.userData.surfaceMesh = brainSurfaceMesh;
    const animate = () => {
      animFrame = requestAnimationFrame(animate);
      const t   = clock.getElapsedTime();
      clock.getDelta(); // consume delta

      // Slow gentle rotation — sync all groups + brain shell
      const rotY = t * 0.07;
      const rotX = Math.sin(t * 0.18) * 0.08;
      nodeGroup.rotation.y     = rotY;
      edgeGroup.rotation.y     = rotY;
      electronGroup.rotation.y = rotY;
      nodeGroup.rotation.x     = rotX;
      edgeGroup.rotation.x     = rotX;
      electronGroup.rotation.x = rotX;

      // Sync brain shell meshes
      const wire    = nodeGroup.userData.wireMesh;
      const surface = nodeGroup.userData.surfaceMesh;
      if (wire)    { wire.rotation.y = rotY;    wire.rotation.x = rotX; }
      if (surface) { surface.rotation.y = rotY; surface.rotation.x = rotX; }
      if (wire)    (wire.material as any).opacity    = powered ? 0.14 : 0.06;
      if (surface) (surface.material as any).opacity = powered ? 0.14 : 0.07;

      // Node pulse
      nodes.forEach(nd => {
        const mat = nd.mesh.material as any;
        if (powered) {
          const p = Math.sin(t * 2.2 + nd.pulse) * 0.35 + 0.65;
          mat.opacity   = Math.max(0.3, Math.min(1, p));
          mat.emissive.setHex(0x003399);
          mat.color.setHex(0x00e5ff);
        } else {
          const p = Math.sin(t * 0.9 + nd.pulse) * 0.12 + 0.38;
          mat.opacity   = Math.max(0.15, Math.min(0.65, p));
          mat.emissive.setHex(0x001133);
          mat.color.setHex(0x00c2ff);
        }
      });

      // Edge opacity
      edges.forEach(e => {
        const mat = e.line.material as any;
        mat.opacity = powered ? 0.30 : 0.08;
      });

      // Electrons
      electrons.forEach(el => {
        el.progress += el.speed * 0.012;
        if (el.progress > 1) {
          el.progress = 0;
          el.edgeIdx  = Math.floor(Math.random() * edges.length);
        }
        const edge    = edges[el.edgeIdx];
        const fromPos = nodes[edge.from].base;
        const toPos   = nodes[edge.to].base;
        el.mesh.position.lerpVectors(fromPos, toPos, el.progress);
        el.mesh.visible = powered;
      });

      renderer.render(scene, camera);
    };
    animate();
  }

  async function spawnElectrons() {
    const THREE = await import('three');
    electronGroup.clear();
    electrons = [];
    if (!edges.length) return;
    for (let i = 0; i < 50; i++) {
      const geo  = new THREE.SphereGeometry(0.022, 6, 6);
      const mat  = new THREE.MeshBasicMaterial({ color: 0x00ffff });
      const mesh = new THREE.Mesh(geo, mat);
      electronGroup.add(mesh);
      electrons.push({ mesh, edgeIdx: Math.floor(Math.random() * edges.length), progress: Math.random(), speed: 0.6 + Math.random() * 1.0 });
    }
  }

  async function powerOn() {
    powered = true;
    startCipher();
    await spawnElectrons();
  }

  function powerOff() {
    powered = false;
    stopCipher();
    electrons.forEach(el => { el.mesh.visible = false; });
  }

  function handleResize() {
    if (!canvasEl || !renderer || !camera) return;
    const w = canvasEl.clientWidth, h = canvasEl.clientHeight;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  }

  onMount(() => { initThree(); window.addEventListener('resize', handleResize); });
  onDestroy(() => { cancelAnimationFrame(animFrame); renderer?.dispose(); window.removeEventListener('resize', handleResize); stopCipher(); });
</script>

<!-- Robotic pointing hand cursor SVG — positioned absolutely inside the container -->
{#if cursorOver}
  <div class="robot-cursor" style="left:{cursorX}px;top:{cursorY}px" aria-hidden="true">
    <svg width="40" height="48" viewBox="0 0 40 48" fill="none" xmlns="http://www.w3.org/2000/svg">
      <!-- Palm -->
      <rect x="10" y="22" width="20" height="18" rx="4" fill="#0a0a1a" stroke="#00c2ff" stroke-width="1.5"/>
      <!-- Index finger pointing -->
      <rect x="17" y="6" width="7" height="18" rx="3.5" fill="#0a0a1a" stroke="#00c2ff" stroke-width="1.5"/>
      <!-- Middle finger (shorter, tucked) -->
      <rect x="25" y="14" width="6" height="12" rx="3" fill="#0a0a1a" stroke="#00c2ff" stroke-width="1.2"/>
      <!-- Ring finger -->
      <rect x="10" y="16" width="6" height="10" rx="3" fill="#0a0a1a" stroke="#00c2ff" stroke-width="1.2"/>
      <!-- Thumb -->
      <rect x="4" y="26" width="8" height="6" rx="3" fill="#0a0a1a" stroke="#00c2ff" stroke-width="1.2"/>
      <!-- Knuckle lines on index -->
      <line x1="17" y1="12" x2="24" y2="12" stroke="#00c2ff" stroke-width="0.8" opacity="0.6"/>
      <line x1="17" y1="17" x2="24" y2="17" stroke="#00c2ff" stroke-width="0.8" opacity="0.6"/>
      <!-- Fingertip glow -->
      <circle cx="20.5" cy="7" r="3" fill="none" stroke="#00e5ff" stroke-width="0.8" opacity="0.8"/>
      <circle cx="20.5" cy="7" r="1.5" fill="#00e5ff" opacity="0.9"/>
      <!-- Mechanical joints highlight -->
      <rect x="10" y="28" width="20" height="1" rx="0.5" fill="#00c2ff" opacity="0.3"/>
      <rect x="10" y="34" width="20" height="1" rx="0.5" fill="#00c2ff" opacity="0.3"/>
    </svg>
  </div>
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
  <canvas bind:this={canvasEl} class="brain-canvas" aria-hidden="true"></canvas>

  <!-- 0 / 1 power buttons -->
  <div class="binary-buttons" aria-label="Neural network power controls">
    <button class="bin-btn bin-btn--off" class:active={!powered} onclick={powerOff} aria-pressed={!powered} title="Power off">0</button>
    <button class="bin-btn bin-btn--on"  class:active={powered}  onclick={powerOn}  aria-pressed={powered}  title="Activate neural network">1</button>
  </div>

  <!-- Cypher terminal -->
  {#if showCipher}
    <div class="cypher-dialog" role="log" aria-live="polite">
      <div class="cypher-header">
        <span class="cypher-dot cypher-dot--red"></span>
        <span class="cypher-dot cypher-dot--yellow"></span>
        <span class="cypher-dot cypher-dot--green"></span>
        <span class="cypher-title">eZeroAndOne — NEURAL OS v2.0.0</span>
      </div>
      <div class="cypher-body">
        {#each cipherLines as line}
          <p class="cypher-line">
            <span class="cypher-prompt">▸</span>{line}<span class="cypher-cursor">█</span>
          </p>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .brain-container {
    position: relative; width: 100%; height: 100%;
    min-height: 480px; cursor: none;
  }

  .brain-canvas { width: 100%; height: 100%; display: block; }

  /* ── Robot cursor ──────────────────────────────────── */
  .robot-cursor {
    position: absolute; pointer-events: none; z-index: 20;
    transform: translate(-8px, -4px);
    filter: drop-shadow(0 0 6px rgba(0,194,255,0.7));
    transition: left 0.04s linear, top 0.04s linear;
  }

  /* ── 0/1 buttons ────────────────────────────────────── */
  .binary-buttons {
    position: absolute; bottom: 2rem; left: 50%;
    transform: translateX(-50%);
    display: flex; gap: 1rem; z-index: 10;
  }

  .bin-btn {
    width: 58px; height: 58px; border-radius: 50%;
    border: 2px solid rgba(255,255,255,0.15);
    background: rgba(0,0,0,0.65); backdrop-filter: blur(8px);
    color: rgba(255,255,255,0.45);
    font-family: 'Inter Tight', monospace; font-size: 1.6rem; font-weight: 900;
    cursor: pointer; display: flex; align-items: center; justify-content: center;
    transition: all 0.3s cubic-bezier(0.16,1,0.3,1); position: relative; overflow: hidden;
  }

  .bin-btn::after {
    content: ''; position: absolute; inset: 0; border-radius: 50%; opacity: 0;
    transition: opacity 0.3s;
  }
  .bin-btn--on::after  { background: radial-gradient(circle, rgba(0,194,255,0.25) 0%, transparent 70%); }
  .bin-btn--off::after { background: radial-gradient(circle, rgba(255,51,102,0.25) 0%, transparent 70%); }

  .bin-btn--on.active {
    border-color: #00c2ff; color: #00e5ff;
    box-shadow: 0 0 28px rgba(0,194,255,0.7), 0 0 56px rgba(0,194,255,0.25);
    background: rgba(0,30,70,0.75);
    animation: pulse-on 2s ease-in-out infinite;
  }
  .bin-btn--off.active {
    border-color: #ff3366; color: #ff6688;
    box-shadow: 0 0 18px rgba(255,51,102,0.5);
    background: rgba(50,0,15,0.75);
  }
  .bin-btn--on.active::after, .bin-btn--off.active::after { opacity: 1; }

  @keyframes pulse-on {
    0%,100% { box-shadow: 0 0 28px rgba(0,194,255,0.7),  0 0 56px rgba(0,194,255,0.25); }
    50%      { box-shadow: 0 0 42px rgba(0,194,255,0.95), 0 0 80px rgba(0,194,255,0.4); }
  }

  /* ── Cypher dialog ──────────────────────────────────── */
  .cypher-dialog {
    position: absolute; top: 1.25rem; right: 0.75rem;
    width: min(290px, 90%);
    background: rgba(0,0,0,0.9); border: 1px solid rgba(0,194,255,0.3);
    border-radius: 10px; overflow: hidden;
    backdrop-filter: blur(20px);
    box-shadow: 0 8px 40px rgba(0,194,255,0.12); z-index: 15;
    animation: slide-in 0.3s cubic-bezier(0.16,1,0.3,1);
  }

  @keyframes slide-in {
    from { opacity: 0; transform: translateY(-10px) scale(0.97); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
  }

  .cypher-header {
    display: flex; align-items: center; gap: 5px; padding: 7px 12px;
    background: rgba(255,255,255,0.03); border-bottom: 1px solid rgba(0,194,255,0.12);
  }
  .cypher-dot { width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0; }
  .cypher-dot--red    { background: #ff5f57; }
  .cypher-dot--yellow { background: #ffbd2e; }
  .cypher-dot--green  { background: #28c940; }
  .cypher-title {
    font-family: 'Inter Tight', monospace; font-size: 0.58rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.06em;
    color: rgba(255,255,255,0.35); margin-left: 4px; white-space: nowrap;
    overflow: hidden; text-overflow: ellipsis;
  }
  .cypher-body {
    padding: 10px 12px; max-height: 220px; overflow-y: auto;
    scrollbar-width: none; display: flex; flex-direction: column; gap: 3px;
  }
  .cypher-body::-webkit-scrollbar { display: none; }
  .cypher-line {
    display: flex; align-items: center; gap: 5px;
    font-family: 'SFMono-Regular', 'Consolas', monospace;
    font-size: 0.62rem; color: #00e5ff; margin: 0;
    animation: type-in 0.15s ease; white-space: nowrap;
  }
  @keyframes type-in {
    from { opacity: 0; transform: translateX(-3px); }
    to   { opacity: 1; transform: translateX(0); }
  }
  .cypher-prompt { color: rgba(0,194,255,0.5); font-size: 0.58rem; flex-shrink: 0; }
  .cypher-cursor { animation: blink 0.8s step-end infinite; color: #00e5ff; font-size: 0.55rem; }
  @keyframes blink { 50% { opacity: 0; } }
</style>
