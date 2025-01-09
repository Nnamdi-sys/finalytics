class CodeTabsComponent extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });

        // Initialize state
        this.languageState = 'rs'; // Default language state (Rust)
        this.currentTab = 'ticker'; // Default tab (Ticker)

        // Define mappings for dynamic updates
        this.codeLinks = {
            ticker: {
                rs: '/code_examples/ticker_rs',
                py: '/code_examples/ticker_py',
            },
            portfolio: {
                rs: '/code_examples/portfolio_rs',
                py: '/code_examples/portfolio_py',
            },
        };

        this.imageLinks = {
            ticker: '../images/ticker.png',
            portfolio: '../images/portfolio.png',
        };

        this.pageLinks = {
            ticker: '/ticker',
            portfolio: '/portfolio',
        };
    }

    connectedCallback() {
        this.render();
        this.addEventListeners();
        this.updateTabButtons();
        this.updateLanguageButtons();
        this.fetchCodeAndImage(this.currentTab, this.languageState).catch(console.error);
    }

    render() {
        this.shadowRoot.innerHTML = `
    <style>
        .container {
            display: flex;
            flex-direction: column;
            padding: 5px;
            margin-top: 0;
            margin-bottom: 10px;
            font-family: 'Poppins', sans-serif;
        }

        h1 {
            text-align: center;
            font-size: 40px;
            font-weight: 700;
            color: #2E7D32;
            margin-bottom: 20px;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
            font-family: 'Poppins', sans-serif;
            background: linear-gradient(90deg, rgba(46,125,50,1) 0%, rgba(76,175,80,1) 50%, rgba(46,125,50,1) 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        .download-stats {
            text-align: center;
            margin-bottom: 20px;
        }

        .download-stats a {
            display: inline-flex;
            align-items: center;
            gap: 10px;
            text-decoration: none;
            color: inherit;
            margin: 0 10px;
        }

        .download-stats img {
            height: 20px;
        }

        p {
            text-align: center;
            font-size: 20px;
            color: #424242;
            line-height: 1.6;
            margin: 0 10px;
            font-family: 'Poppins', sans-serif;
            padding: 10px;
        }

        .tabs {
            display: flex;
            margin-bottom: 10px;
        }

        .tab-button {
            flex: 1;
            padding: 10px;
            background-color: #f0f0f0;
            border: none;
            cursor: pointer;
            text-align: center;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 10px;
            font-weight: bold;
            color: #2E7D32;
            font-size: 16px;
        }

        .tab-button.active {
            background-color: #d0d0d0;
            font-weight: bold;
        }

        .content {
            display: flex;
            gap: 20px;
            flex-direction: column;
        }

        .code-container {
            flex: 1;
            border: 1px solid #ccc;
            padding: 10px;
            overflow-x: auto;
            background-color: #2b303b;
            color: #fff;
            display: flex;
            flex-direction: column;
            flex-grow: 1;
        }

        .code-tabs {
            display: flex;
            gap: 5px;
            margin-bottom: 10px;
        }

        .code-tab-button {
            padding: 5px 10px;
            border: none;
            cursor: pointer;
            font-weight: bold;
            color: #fff;
            font-size: 14px;
            border-radius: 4px;
            background-color: #333;
        }

        .code-tab-button.active {
            background-color: #2E7D32;
            color: #fff;
        }

        .image-container {
            flex: 1;
            text-align: center;
            position: relative;
        }

        .image-container img {
            max-width: 100%;
            height: auto;
        }

        .image-tabs {
            display: flex;
            justify-content: flex-start; /* Justify left */
            gap: 10px;
            margin-bottom: 10px;
        }

        .image-tab-button {
            padding: 5px 10px;
            border: none;
            cursor: pointer;
            font-weight: bold;
            color: #2E7D32;
            font-size: 16px;
            background-color: #f0f0f0;
        }

        .image-tab-button.active {
            background-color: #d0d0d0;
            font-weight: bold;
        }

        @media (min-width: 768px) {
            .content {
                flex-direction: row;
            }
        }
    </style>

    <!-- Add Links to Icon Libraries (Bootstrap Icons + Devicons) -->
    <link href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.7.2/font/bootstrap-icons.css" rel="stylesheet">
    <link href="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" rel="stylesheet">

    <div class="container">
        <h1>Financial Analytics Powered by Rust</h1>
        <div class="download-stats">
            <a href="https://crates.io/crates/finalytics">
                <img src="https://img.shields.io/crates/v/finalytics" alt="Crates.io version badge">
                <img src="https://img.shields.io/crates/d/finalytics?color=orange" alt="Crates.io download stats">
            </a>
            <a href="https://pypi.org/project/finalytics/">
                <img src="https://img.shields.io/pypi/v/finalytics?color=blue" alt="PyPI version badge">
                <img src="https://static.pepy.tech/personalized-badge/finalytics?period=total&units=international_system&left_color=black&right_color=blue&left_text=Downloads" alt="PePy download stats">
            </a>
        </div>
        <p>Finalytics leverages Rust’s high performance with a Python integration, offering robust tools for financial data analysis.</p>
    </div>

    <div class="container">
        <div class="content">
            <!-- Code Container -->
            <div class="code-container">
                <div class="code-tabs">
                    <button class="code-tab-button" id="rust-tab">
                        <i class="devicon-rust-plain me-2"></i>
                        Rust
                    </button>
                    <button class="code-tab-button" id="python-tab">
                        <i class="devicon-python-plain me-2"></i>
                        Python
                    </button>
                </div>
                <pre id="code-content">Loading...</pre>
            </div>

            <!-- Image Container -->
            <div class="image-container">
                <div class="image-tabs">
                    <button class="image-tab-button" id="ticker-tab">
                        <i class="bi bi-graph-up me-2"></i>
                        Ticker
                    </button>
                    <button class="image-tab-button" id="portfolio-tab">
                        <i class="bi bi-pie-chart me-2"></i>
                        Portfolio
                    </button>
                </div>
                <a id="image-link" target="_blank">
                    <img id="image" alt="Chart">
                </a>
            </div>
        </div>
    </div>
    `;
    }

    addEventListeners() {
        // Tab switching for main tabs
        this.shadowRoot.getElementById('ticker-tab').addEventListener('click', () => {
            this.updateActiveTab('ticker');
        });

        this.shadowRoot.getElementById('portfolio-tab').addEventListener('click', () => {
            this.updateActiveTab('portfolio');
        });

        // Tab switching for language tabs
        this.shadowRoot.getElementById('rust-tab').addEventListener('click', () => {
            this.updateActiveLanguage('rs');
        });

        this.shadowRoot.getElementById('python-tab').addEventListener('click', () => {
            this.updateActiveLanguage('py');
        });
    }

    updateActiveTab(tabId) {
        this.currentTab = tabId;
        this.updateTabButtons();
        this.fetchCodeAndImage(this.currentTab, this.languageState).catch(console.error);
    }

    updateActiveLanguage(language) {
        this.languageState = language;
        this.updateLanguageButtons();
        this.fetchCodeAndImage(this.currentTab, this.languageState).catch(console.error);
    }

    updateTabButtons() {
        this.shadowRoot.querySelectorAll('.tabs .tab-button').forEach(button => button.classList.remove('active'));
        const activeButtonId = this.currentTab === 'ticker' ? 'ticker-tab' : 'portfolio-tab';
        this.shadowRoot.getElementById(activeButtonId).classList.add('active');
        this.shadowRoot.querySelectorAll('.image-tabs .image-tab-button').forEach(button => button.classList.remove('active'));
        this.shadowRoot.getElementById(activeButtonId).classList.add('active');
    }

    updateLanguageButtons() {
        this.shadowRoot.querySelectorAll('.code-container .code-tab-button').forEach(button => button.classList.remove('active'));
        const activeButtonId = this.languageState === 'rs' ? 'rust-tab' : 'python-tab';
        this.shadowRoot.getElementById(activeButtonId).classList.add('active');
    }

    async fetchCodeAndImage(tabId, language) {
        const codeUrl = this.codeLinks[tabId][language];
        const imageUrl = this.imageLinks[tabId];
        const pageUrl = this.pageLinks[tabId];

        try {
            // Fetch code content
            const codeResponse = await fetch(codeUrl);

            this.shadowRoot.getElementById('code-content').innerHTML = await codeResponse.text();

            // Dynamically update image and link
            const imageElement = this.shadowRoot.getElementById('image');
            const linkElement = this.shadowRoot.getElementById('image-link');

            imageElement.src = imageUrl;
            linkElement.href = pageUrl;
        } catch (error) {
            console.error('Error fetching content:', error);
        }
    }
}

customElements.define('code-tabs', CodeTabsComponent);