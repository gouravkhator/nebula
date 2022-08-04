import LanguageSelectorLogic from "./LanguageSelector.logic";
import "./language_selector_style.scss";

function LanguageSelector({ languagesList }) {
  const { handleLanguageCheckedChange, saveSelectedLanguages } =
    LanguageSelectorLogic({ languagesList });

  return (
    <>
      <fieldset id="language-selector-section">
        <legend>
          Select the languages for which you want this app to be configured
          with:
        </legend>

        {languagesList.map((language, _) => (
          <div>
            <input
              type="checkbox"
              id={language}
              name={language}
              onChange={handleLanguageCheckedChange}
            />
            <label for={language}>
              {language.charAt(0).toUpperCase() +
                language.slice(1).toLowerCase()}
            </label>
          </div>
        ))}
      </fieldset>

      <button onClick={saveSelectedLanguages}>Save Preferences</button>
    </>
  );
}

export default LanguageSelector;
