<script lang="ts">
  /**
   * Full Material Icons Outlined picker with live search.
   * Renders a searchable grid of ~800 common Material Icons.
   * The picker panel uses position:fixed so it works correctly inside
   * scrollable modals or overflow:hidden containers.
   */
  import { tick } from 'svelte';

  interface Props {
    value: string;
    onchange?: (icon: string) => void;
  }

  let { value = $bindable('star'), onchange }: Props = $props();

  let searchQuery = $state('');
  let showPicker = $state(false);

  // Anchor element — used to position the fixed panel beneath the trigger
  let triggerEl = $state<HTMLDivElement | null>(null);
  // Computed panel position
  let panelStyle = $state('');

  // Complete list of common Material Icons (ligature names)
  const ALL_ICONS: string[] = [
    // Navigation & UI
    'home','menu','close','search','settings','more_vert','more_horiz',
    'arrow_back','arrow_forward','arrow_upward','arrow_downward',
    'chevron_left','chevron_right','expand_more','expand_less',
    'first_page','last_page','refresh','sync','replay','undo','redo',
    'check','check_circle','cancel','block','remove','add','edit',
    'delete','delete_forever','clear','done','done_all','drag_handle',
    'open_in_new','launch','link','link_off','share','save','print',
    'download','upload','cloud_upload','cloud_download','attach_file',
    'filter_list','sort','swap_vert','swap_horiz','compare_arrows',
    // Communication
    'email','mail','send','inbox','drafts','forum','chat','message',
    'chat_bubble','comment','feedback','notifications','notification_important',
    'phone','call','voicemail','contacts','contact_phone','contact_mail',
    'people','person','person_add','person_remove','group','groups',
    'supervisor_account','manage_accounts','badge','account_circle',
    // Content & Media
    'article','description','assignment','note','notes','subject',
    'list','playlist_add','playlist_check','library_books','library_add',
    'bookmark','bookmarks','label','tag','new_label','flag','tour',
    'image','photo','photo_camera','camera_alt','videocam','video_call',
    'mic','volume_up','volume_off','headphones','music_note','queue_music',
    'play_arrow','pause','stop','skip_next','skip_previous','replay_10',
    'movie','live_tv','tv','cast','airplay','screen_share','present_to_all',
    'crop','rotate_right','flip','photo_filter','auto_fix_high','brush',
    'palette','format_paint','color_lens','style','gesture','create',
    'content_copy','content_cut','content_paste','find_replace','spellcheck',
    'format_bold','format_italic','format_underlined','format_list_bulleted',
    'format_list_numbered','format_quote','code','terminal','data_object',
    // Technology & Dev
    'computer','laptop','tablet','phone_android','phone_iphone','watch',
    'keyboard','mouse','monitor','desktop_windows','memory','storage',
    'cloud','cloud_circle','cloud_done','cloud_queue','cloud_sync',
    'dns','router','wifi','wifi_off','bluetooth','bluetooth_connected',
    'cast_connected','data_usage','network_check','signal_wifi_4_bar',
    'api','integration_instructions','developer_mode','code_off',
    'bug_report','build','construction','engineering','handyman','hardware',
    'settings_ethernet','settings_system_daydream','developer_board',
    'memory_alt','data_array','data_thresholding','schema','database',
    'table_view','grid_view','view_list','view_module','view_quilt',
    'hub','lan','mediation','device_hub','cable','usb','sd_card',
    // Business & Finance
    'work','work_outline','business','business_center','domain',
    'apartment','store','storefront','shopping_cart','shopping_bag',
    'point_of_sale','receipt','receipt_long','payments','credit_card',
    'account_balance','account_balance_wallet','savings','currency_exchange',
    'trending_up','trending_down','bar_chart','pie_chart','stacked_bar_chart',
    'analytics','insights','assessment','show_chart','candlestick_chart',
    'monetization_on','attach_money','money','euro','price_change',
    'inventory','inventory_2','warehouse','local_shipping','delivery_dining',
    'campaign','ads_click','sell','new_releases','star','star_border',
    // Science & Education
    'science','biotech','psychology','psychology_alt','neurology',
    'calculate','functions','category','class','school',
    'auto_stories','menu_book','import_contacts','local_library',
    'workspace_premium','emoji_events','military_tech','verified',
    'grade','stars','celebration',
    // Health & Medical
    'health_and_safety','medical_services','local_hospital','emergency',
    'vaccines','medication','thermostat','fitness_center','self_improvement',
    'spa','face','accessibility','accessible','wheelchair_pickup',
    // Location & Travel
    'location_on','location_off','my_location','place','map','navigation',
    'directions','directions_car','flight','train','bus_alert','local_taxi',
    'hotel','restaurant','local_cafe','local_bar','shopping_mall',
    'park','beach_access','pool','sports','sports_soccer','sports_basketball',
    'terrain','landscape','forest','water','waves','wb_sunny','ac_unit',
    // Security & Privacy
    'security','lock','lock_open','https','gpp_good','gpp_bad','gpp_maybe',
    'shield','verified_user','admin_panel_settings',
    'key','password','fingerprint','policy',
    'no_encryption','enhanced_encryption',
    // Alerts & Status
    'info','warning','error','error_outline','report','report_problem',
    'help','help_outline','priority_high',
    'circle','radio_button_checked','radio_button_unchecked','toggle_on',
    'toggle_off','power','power_off','flash_on','flash_off','offline_bolt',
    'hourglass_empty','hourglass_full','timer','timer_off','access_time',
    'schedule','event','event_available','event_busy','date_range',
    // Productivity & Tools
    'task','task_alt','checklist','fact_check',
    'playlist_add_check','rule','grading','rate_review',
    'sticky_note_2','post_add','edit_note','draw',
    'text_fields','title','short_text','wrap_text','translate',
    'language','public','explore','travel_explore',
    'rocket_launch','rocket','satellite','bolt','electric_bolt',
    'electrical_services','connecting_airports',
    // Architecture & Design
    'architecture','design_services','foundation','roofing',
    'deck','meeting_room','stairs','elevator','door_front',
    'window','garage','fence','yard','grass','local_florist',
    'light_mode','dark_mode','brightness_6',
    // Misc & Symbols
    'favorite','favorite_border','thumb_up','thumb_down','sentiment_satisfied',
    'sentiment_dissatisfied','mood','mood_bad','manage_search',
    'fullscreen','fullscreen_exit','picture_in_picture','fit_screen',
    'aspect_ratio','straighten','all_inclusive','autorenew','loop','repeat',
    'shuffle','compare','difference','merge','call_split','call_merge',
    'qr_code','qr_code_2','barcode','nfc',
  ];

  const filtered = $derived(
    searchQuery.trim().length === 0
      ? ALL_ICONS
      : ALL_ICONS.filter(icon =>
          icon.includes(searchQuery.toLowerCase().replace(/\s+/g, '_'))
        )
  );

  async function openPicker() {
    showPicker = true;
    await tick();
    repositionPanel();
  }

  function repositionPanel() {
    if (!triggerEl) return;
    const rect = triggerEl.getBoundingClientRect();
    const spaceBelow = window.innerHeight - rect.bottom;
    const spaceAbove = rect.top;
    const panelH = 360;

    if (spaceBelow >= panelH || spaceBelow >= spaceAbove) {
      // open downward
      panelStyle = `top:${rect.bottom + 6}px;left:${rect.left}px;width:${rect.width}px;`;
    } else {
      // open upward
      panelStyle = `bottom:${window.innerHeight - rect.top + 6}px;left:${rect.left}px;width:${rect.width}px;`;
    }
  }

  function togglePicker() {
    if (showPicker) {
      showPicker = false;
    } else {
      openPicker();
    }
  }

  function select(icon: string) {
    value = icon;
    onchange?.(icon);
    showPicker = false;
    searchQuery = '';
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && showPicker) {
      showPicker = false;
      searchQuery = '';
    }
  }

  // Close when clicking outside the picker panel
  function handleWindowClick(e: MouseEvent) {
    if (!showPicker) return;
    const target = e.target as Node;
    const panel = document.getElementById('mip-panel');
    if (triggerEl && !triggerEl.contains(target) && panel && !panel.contains(target)) {
      showPicker = false;
      searchQuery = '';
    }
  }
</script>

<svelte:window onkeydown={handleWindowKeydown} onclick={handleWindowClick} />

<div class="icon-picker-wrap">
  <!-- Current selection + toggle trigger -->
  <div
    class="icon-selected"
    role="button"
    tabindex="0"
    bind:this={triggerEl}
    onclick={togglePicker}
    onkeydown={(e) => e.key === 'Enter' && togglePicker()}
  >
    <span class="material-icons-outlined preview-icon">{value || 'star'}</span>
    <span class="preview-name">{value || 'star'}</span>
    <span class="material-icons-outlined toggle-chevron">{showPicker ? 'expand_less' : 'expand_more'}</span>
  </div>

  <!-- Direct text entry for any icon name -->
  <input
    type="text"
    class="direct-input"
    placeholder="or type any icon name…"
    bind:value
    oninput={() => onchange?.(value)}
  />
</div>

<!-- Picker panel rendered at document root via fixed positioning to escape modal overflow clipping -->
{#if showPicker}
  <div
    id="mip-panel"
    class="picker-panel"
    role="dialog"
    aria-label="Icon picker"
    style={panelStyle}
  >
    <div class="picker-search">
      <span class="material-icons-outlined search-icon">search</span>
      <input
        type="text"
        class="search-input"
        placeholder="Search icons…"
        bind:value={searchQuery}
        autofocus
      />
      {#if searchQuery}
        <button
          class="clear-search"
          type="button"
          onclick={() => (searchQuery = '')}
          aria-label="Clear search"
        >
          <span class="material-icons-outlined">close</span>
        </button>
      {/if}
    </div>

    <div class="picker-count">{filtered.length} icon{filtered.length !== 1 ? 's' : ''}</div>

    <div class="picker-grid">
      {#each filtered as icon (icon)}
        <button
          type="button"
          class="icon-btn"
          class:active={value === icon}
          onclick={() => select(icon)}
          title={icon.replace(/_/g, ' ')}
          aria-label={icon.replace(/_/g, ' ')}
          aria-pressed={value === icon}
        >
          <span class="material-icons-outlined">{icon}</span>
        </button>
      {/each}
      {#if filtered.length === 0}
        <p class="no-results">No icons match "{searchQuery}"</p>
      {/if}
    </div>
  </div>
{/if}

<style>
  .icon-picker-wrap {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  /* ── Selected display ─────────────────────────────────────── */
  .icon-selected {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    background: rgba(255,255,255,0.05);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
    user-select: none;
  }

  .icon-selected:hover,
  .icon-selected:focus-visible {
    border-color: rgba(0,194,255,0.5);
    background: rgba(0,194,255,0.06);
    outline: none;
  }

  .preview-icon {
    font-size: 1.5rem;
    color: #00C2FF;
    flex-shrink: 0;
  }

  .preview-name {
    flex: 1;
    font-size: 0.8rem;
    color: rgba(255,255,255,0.7);
    font-family: monospace;
  }

  .toggle-chevron {
    font-size: 1.1rem;
    color: rgba(255,255,255,0.4);
  }

  /* ── Direct text input ────────────────────────────────────── */
  .direct-input {
    background: rgba(255,255,255,0.04) !important;
    border: 1px solid rgba(255,255,255,0.08) !important;
    border-radius: 8px !important;
    padding: 7px 12px !important;
    color: rgba(255,255,255,0.6) !important;
    font-size: 0.8rem !important;
    font-family: monospace !important;
    width: 100% !important;
    box-sizing: border-box !important;
  }

  .direct-input:focus {
    border-color: rgba(0,194,255,0.4) !important;
    outline: none !important;
  }

  /* ── Picker panel — fixed so it escapes overflow:hidden/auto containers ── */
  :global(#mip-panel) {
    position: fixed;
    z-index: 9999;
    background: #0d0d1a;
    border: 1px solid rgba(255,255,255,0.14);
    border-radius: 12px;
    box-shadow: 0 20px 60px rgba(0,0,0,0.8);
    display: flex;
    flex-direction: column;
    max-height: 360px;
    overflow: hidden;
  }

  /* Search bar */
  :global(#mip-panel .picker-search) {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    border-bottom: 1px solid rgba(255,255,255,0.08);
    flex-shrink: 0;
  }

  :global(#mip-panel .search-icon) {
    font-size: 1.1rem;
    color: rgba(255,255,255,0.3);
    flex-shrink: 0;
  }

  :global(#mip-panel .search-input) {
    flex: 1;
    background: transparent !important;
    border: none !important;
    color: #f0f0f0 !important;
    font-size: 0.875rem !important;
    padding: 0 !important;
    outline: none !important;
    width: auto !important;
    box-shadow: none !important;
    min-width: 0;
  }

  :global(#mip-panel .clear-search) {
    background: none;
    border: none;
    color: rgba(255,255,255,0.3);
    cursor: pointer;
    padding: 2px;
    display: flex;
    align-items: center;
    border-radius: 4px;
  }

  :global(#mip-panel .clear-search:hover) { color: rgba(255,255,255,0.7); }

  :global(#mip-panel .clear-search .material-icons-outlined) { font-size: 1rem; }

  /* Count */
  :global(#mip-panel .picker-count) {
    padding: 4px 12px;
    font-size: 0.68rem;
    color: rgba(255,255,255,0.25);
    flex-shrink: 0;
    border-bottom: 1px solid rgba(255,255,255,0.05);
  }

  /* Icon grid */
  :global(#mip-panel .picker-grid) {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(42px, 1fr));
    gap: 2px;
    padding: 8px;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: rgba(255,255,255,0.1) transparent;
  }

  :global(#mip-panel .picker-grid::-webkit-scrollbar) { width: 4px; }
  :global(#mip-panel .picker-grid::-webkit-scrollbar-track) { background: transparent; }
  :global(#mip-panel .picker-grid::-webkit-scrollbar-thumb) {
    background: rgba(255,255,255,0.1);
    border-radius: 4px;
  }

  :global(#mip-panel .icon-btn) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 42px;
    height: 42px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 7px;
    cursor: pointer;
    color: rgba(255,255,255,0.55);
    transition: all 0.12s;
    padding: 0;
  }

  :global(#mip-panel .icon-btn:hover) {
    background: rgba(255,255,255,0.07);
    color: #fff;
    border-color: rgba(255,255,255,0.1);
  }

  :global(#mip-panel .icon-btn.active) {
    background: rgba(0,194,255,0.15);
    color: #00C2FF;
    border-color: rgba(0,194,255,0.4);
  }

  :global(#mip-panel .icon-btn .material-icons-outlined) { font-size: 1.3rem; }

  :global(#mip-panel .no-results) {
    grid-column: 1 / -1;
    text-align: center;
    color: rgba(255,255,255,0.3);
    font-size: 0.8rem;
    padding: 24px;
    margin: 0;
  }
</style>
