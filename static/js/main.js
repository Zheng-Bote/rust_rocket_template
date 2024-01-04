/**
 * @name main.js
 * @summary main js file
 * @version 0.1.0
 * @author ZHENG Robert
 */

window.onload = start;

const defaultLocale = "en";
const secondLocale = "de";
let locale = "";

function start() {
  setFooter();
  setHeader();
  darkMode();
  setBrowserLanguage();

  document
    .getElementById("btn_language")
    .addEventListener("click", function () {
      toggleLocale();
    });
}

function setHeader() {
  document
    .getElementById("brand_button")
    .addEventListener("click", function () {
      window.location.href = "/";
    });
}
function setFooter() {
  const VERSION = "v00.01.00";
  const CREATED = "2021";
  const DATE = new Date();
  const FULLYEAR = DATE.getFullYear();
  let years = "";

  CREATED == FULLYEAR ? (years = CREATED) : (years = CREATED + "-" + FULLYEAR);

  document.getElementById(
    "footer"
  ).innerHTML = `&copy; ZHENG Robert ${years}, ${VERSION}`;
  document.getElementById("footer").addEventListener("click", function () {
    window.location.href = "#body";
  });
}

function toggleLocale() {
  if (locale === defaultLocale) {
    newLocale = secondLocale;
  } else {
    newLocale = defaultLocale;
  }
  setLocale(newLocale);
}

async function chooseLocale() {
  setLocale(defaultLocale);
}
async function setLocale(newLocale) {
  if (newLocale === locale) {
    return;
  }
  const newTranslations = await fetchTranslationsFor(newLocale);
  locale = newLocale;
  translations = newTranslations;
  translatePage();
}
async function fetchTranslationsFor(newLocale) {
  const response = await fetch(`/assets/i18n/${newLocale}.json`);
  return await response.json();
}
function translatePage() {
  document.querySelectorAll("[data-i18n-key]").forEach((item) => {
    translateItem(item);
  });
}
function translateItem(element) {
  const key = element.getAttribute("data-i18n-key");
  const translation = translations[key];
  element.innerText = translation;
}

function setBrowserLanguage() {
  let newLocale = "";
  let userLang = navigator.language || navigator.userLanguage();
  userLang = userLang.split("-")[0];
  userLang !== secondLocale ? (newLocale = "en") : (newLocale = "de");
  setLocale(newLocale);
}

function darkMode() {
  let storedTheme =
    localStorage.getItem("theme") ||
    (window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light");
  if (storedTheme) {
    document.documentElement.setAttribute("data-theme", storedTheme);
    let item = document.getElementById(storedTheme);
    item.style.display = "inline";
  }
}

let toggle = document.getElementById("theme-toggle");
toggle.onclick = function () {
  let currentTheme = document.documentElement.getAttribute("data-theme");
  let targetTheme = "light";

  if (currentTheme === "light") {
    targetTheme = "dark";
    let dark = document.getElementById("dark");
    dark.style.display = "inline";
    let light = document.getElementById("light");
    light.style.display = "none";
  } else {
    let dark = document.getElementById("dark");
    dark.style.display = "none";
    let light = document.getElementById("light");
    light.style.display = "inline";
  }

  document.documentElement.setAttribute("data-theme", targetTheme);
  localStorage.setItem("theme", targetTheme);
};
