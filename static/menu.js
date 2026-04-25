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

  function init() {
    initNavScroll();
    var btn = document.getElementById('mobile-menu-toggle');
    var menu = document.getElementById('mobile-menu');
    if (!btn || !menu) return;

    function setOpen(open) {
      btn.setAttribute('aria-expanded', open ? 'true' : 'false');
      menu.setAttribute('aria-hidden', open ? 'false' : 'true');
      if (open) {
        menu.classList.remove('hidden');
      } else {
        menu.classList.add('hidden');
      }
    }

    btn.addEventListener('click', function () {
      setOpen(btn.getAttribute('aria-expanded') !== 'true');
    });

    // Close on Escape
    document.addEventListener('keydown', function (e) {
      if (e.key === 'Escape') setOpen(false);
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
