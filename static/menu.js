// Mobile menu toggle. Tiny, self-contained, no framework.
// Runs once on DOMContentLoaded. Matches the production React site's
// hamburger → drawer interaction.
(function () {
  'use strict';

  // Toggle nav styling based on scroll position — matches the production
  // React site, which starts the nav transparent and snaps to the white-blur
  // state after any scroll. Keeps hero visual parity.
  function initNavScroll() {
    var nav = document.getElementById('site-nav');
    if (!nav) return;
    var onScroll = function () {
      var scrolled = window.scrollY > 10;
      if (scrolled) {
        nav.classList.remove('bg-transparent', 'border-transparent', 'py-5');
        nav.classList.add('bg-white/90', 'backdrop-blur-md', 'border-border/50', 'py-3', 'shadow-sm');
      } else {
        nav.classList.add('bg-transparent', 'border-transparent', 'py-5');
        nav.classList.remove('bg-white/90', 'backdrop-blur-md', 'border-border/50', 'py-3', 'shadow-sm');
      }
    };
    window.addEventListener('scroll', onScroll, { passive: true });
    onScroll();
  }

  // Auto-open the <details> inside the section the URL hash points at,
  // so /services#cyber-security lands the prospect at an already-
  // expanded card instead of a closed summary they have to click. No-op
  // when there's no hash, no matching section, or no <details> child.
  function openDetailsForHash() {
    if (!location.hash || location.hash.length < 2) return;
    var id;
    try { id = decodeURIComponent(location.hash.slice(1)); } catch (e) { return; }
    var section = document.getElementById(id);
    if (!section) return;
    var details = section.querySelector('details');
    if (details && !details.open) details.open = true;
  }

  function init() {
    // Mark <html> as JS-enabled so the animations.css `.js-on` rules
    // kick in. Without this, `.reveal` stays at its default
    // `opacity: 1` — the baseline ensures no-JS users always see
    // every section, not a hero plus a giant blank gap.
    document.documentElement.classList.add('js-on');
    initNavScroll();
      openDetailsForHash();
    window.addEventListener('hashchange', openDetailsForHash);
    var btn = document.getElementById('mobile-menu-toggle');
    var menu = document.getElementById('mobile-menu');
    if (!btn || !menu) return;

    // `restoreFocus` matters more than it looks. Closing the drawer puts
    // `display: none` on an ancestor of whatever is focused, and the browser
    // responds by dropping focus to <body>. A keyboard user who opened the
    // menu, changed their mind and pressed Escape was left at the very top of
    // the document with no visible focus, having to tab through the whole
    // header again. Measured before this: Escape closed the drawer and
    // activeElement became <body>.
    function setOpen(open, restoreFocus) {
      btn.setAttribute('aria-expanded', open ? 'true' : 'false');
      menu.setAttribute('aria-hidden', open ? 'false' : 'true');
      if (open) {
        menu.classList.remove('hidden');
      } else {
        menu.classList.add('hidden');
        if (restoreFocus) btn.focus();
      }
    }

    btn.addEventListener('click', function () {
      setOpen(btn.getAttribute('aria-expanded') !== 'true');
    });

    // Close on Escape, returning focus to the control that opened it.
    document.addEventListener('keydown', function (e) {
      if (e.key !== 'Escape') return;
      if (btn.getAttribute('aria-expanded') !== 'true') return;
      // Only take focus back if it is currently inside the thing being
      // closed. Escape pressed while typing in a form field should not yank
      // the caret up to the menu button.
      var inside = menu.contains(document.activeElement);
      setOpen(false, inside);
    });

    // Close when clicking any link inside the menu
    menu.addEventListener('click', function (e) {
      if (e.target && e.target.tagName === 'A') setOpen(false);
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
