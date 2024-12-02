import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [searchQuery, setSearchQuery] = useState("");

  const handleSearch = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && searchQuery.trim()) {
      try {
        await invoke("open_search", { query: searchQuery.trim() });
        setSearchQuery(""); // 清空输入框
      } catch (error) {
        console.error("搜索出错:", error);
      }
    }
  };

  return (
    <div className="search-box" data-tauri-drag-region>
      <input
        type="text"
        value={searchQuery}
        onChange={(e) => setSearchQuery(e.target.value)}
        onKeyDown={handleSearch}
        placeholder="输入搜索内容并按回车..."
        className="search-input"
        autoFocus
      />
    </div>
  );
}

export default App;
