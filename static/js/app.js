/* SurfContext — Vanilla JS */

// Global copyCode for spec page inline onclick handlers
function copyCode(btn) {
    var codeBlock = btn.closest('.code-block');
    var code = codeBlock.querySelector('code');
    var text = code.textContent;
    navigator.clipboard.writeText(text).then(function() {
        var span = btn.querySelector('span');
        if (span) {
            var original = span.textContent;
            span.textContent = 'Copied';
            setTimeout(function() { span.textContent = original; }, 2000);
        }
    });
}

(function () {
    "use strict";

    /* --- Theme --- */

    function getPreferredTheme() {
        var saved = localStorage.getItem("surfcontext-theme");
        if (saved) return saved;
        return window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";
    }

    function applyTheme(theme) {
        document.documentElement.setAttribute("data-theme", theme);
        localStorage.setItem("surfcontext-theme", theme);
    }

    applyTheme(getPreferredTheme());

    /* --- DOM Ready --- */

    document.addEventListener("DOMContentLoaded", function () {

        /* Theme toggle */
        var themeBtn = document.querySelector(".nav-theme-toggle");
        if (themeBtn) {
            themeBtn.addEventListener("click", function () {
                var current = document.documentElement.getAttribute("data-theme");
                applyTheme(current === "dark" ? "light" : "dark");
            });
        }

        /* Mobile menu */
        var navToggle = document.querySelector(".nav-toggle");
        var navLinks = document.querySelector(".nav-links");

        if (navToggle && navLinks) {
            navToggle.addEventListener("click", function (e) {
                e.stopPropagation();
                var open = navLinks.classList.toggle("open");
                navToggle.setAttribute("aria-expanded", open);
            });

            navLinks.querySelectorAll("a").forEach(function (link) {
                link.addEventListener("click", function () {
                    navLinks.classList.remove("open");
                    navToggle.setAttribute("aria-expanded", "false");
                });
            });

            document.addEventListener("click", function (e) {
                if (!navToggle.contains(e.target) && !navLinks.contains(e.target)) {
                    navLinks.classList.remove("open");
                    navToggle.setAttribute("aria-expanded", "false");
                }
            });
        }

        /* Code copy buttons */
        document.querySelectorAll(".code-block").forEach(function (block) {
            if (block.querySelector(".code-copy")) return;
            var btn = document.createElement("button");
            btn.className = "code-copy";
            btn.textContent = "Copy";
            btn.addEventListener("click", function () {
                var code = block.querySelector("code");
                if (!code) return;
                navigator.clipboard.writeText(code.textContent).then(function () {
                    btn.textContent = "Copied!";
                    setTimeout(function () { btn.textContent = "Copy"; }, 2000);
                });
            });
            block.appendChild(btn);
        });

        /* Spec TOC tracking */
        var toc = document.querySelector(".spec-toc");
        if (toc) {
            var sections = document.querySelectorAll(".spec-section");
            var tocItems = toc.querySelectorAll(".toc-item");
            if (sections.length && tocItems.length) {
                var observer = new IntersectionObserver(function (entries) {
                    entries.forEach(function (entry) {
                        if (entry.isIntersecting) {
                            var id = entry.target.getAttribute("id");
                            tocItems.forEach(function (item) {
                                item.classList.toggle("active", item.getAttribute("href") === "#" + id);
                            });
                        }
                    });
                }, { rootMargin: "-80px 0px -60% 0px", threshold: 0 });
                sections.forEach(function (s) { observer.observe(s); });
            }
        }

        /* Stack selector (getting-started page) */
        var stackSelector = document.querySelector(".stack-selector");
        if (stackSelector) {
            var hidden = stackSelector.parentElement.querySelector("input[type=hidden]");
            stackSelector.querySelectorAll(".stack-btn").forEach(function (btn) {
                btn.addEventListener("click", function () {
                    btn.classList.toggle("selected");
                    if (hidden) {
                        var selected = stackSelector.querySelectorAll(".stack-btn.selected");
                        var vals = [];
                        selected.forEach(function (s) { vals.push(s.dataset.value || s.textContent.trim()); });
                        hidden.value = vals.join(",");
                    }
                });
            });
        }
    });
})();
