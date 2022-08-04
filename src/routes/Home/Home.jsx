import "./home_style.scss";
import LanguageSelector from "../../components/LanguageSelector/LanguageSelector";

function Home() {
  const languagesList = ["javascript", "java", "python", "cpp"];

  return (
    <div className="container" id="home-page">
      <h1>Tauri Demo App</h1>

      <h2>Welcome to Tauri Demo App</h2>

      <LanguageSelector languagesList={languagesList} />

      <p className="section-notes">
        Note: You can add the setup of more languages or remove the setup of
        above selected languages
      </p>
    </div>
  );
}

export default Home;
