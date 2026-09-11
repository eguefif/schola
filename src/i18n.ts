import { createI18n } from "vue-i18n";
import fr from "./fr.json";

const messages = { fr };

export const i18n = createI18n({
  locale: "fr",
  fallbackLocale: "fr",
  messages: messages,
});
