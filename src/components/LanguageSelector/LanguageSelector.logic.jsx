import { useEffect, useState } from "react";

function LanguageSelectorLogic({ languagesList }) {
  const [userLanguagePref, setUserLanguagePref] = useState({});

  useEffect(() => {
    const tempUserLangMap = userLanguagePref;

    languagesList.forEach((language) => {
      tempUserLangMap[language] = false;
    });

    setUserLanguagePref((prevUserLangPref) => ({
      ...prevUserLangPref,
      ...tempUserLangMap,
    }));
  }, [languagesList]);

  const handleLanguageCheckedChange = (e) => {
    if (typeof e.target.id !== "string" || e.target.id === "") {
      return;
    }

    setUserLanguagePref((prevUserLangPref) => ({
      ...prevUserLangPref,
      [e.target.id]: e.target.checked,
    }));
  };

  const saveSelectedLanguages = (e) => {
    // TODO: save the keys of userLanguagePref which have value as true, through the rust to the user settings json file
    const selectedLanguages = Object.keys(userLanguagePref).filter(
      (language) => {
        return userLanguagePref[language];
      }
    );

    if (selectedLanguages.length === 0) {
      /*
      TODO: set the message as
      "As you didn't select any languages, we are proceeding with the base setup of formatter, and other programming helpers.."
      */
    }

    console.log({ selectedLanguages });
    // TODO: invoke the rust function for setting the language preferences
  };

  return {
    handleLanguageCheckedChange,
    saveSelectedLanguages,
  };
}

export default LanguageSelectorLogic;
