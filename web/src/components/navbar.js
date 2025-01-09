class NavbarComponent extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.innerHTML = `
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
            <div class="container-fluid">
                <a class="navbar-brand" href="/">
                    <img src="../images/logo.svg" width="200" height="50" class="d-inline-block align-top" alt="Logo">
                </a>
                <div class="collapse navbar-collapse">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item">
                            <a class="nav-link d-flex align-items-center" style="color: blue; font-weight: bold;" href="https://github.com/Nnamdi-sys/finalytics" target="_blank">
                                <i class="bi bi-github me-2"></i>Docs
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link d-flex align-items-center" style="color: blue; font-weight: bold; text-decoration: none;" href="https://docs.rs/finalytics/" target="_blank">
                                <i class="devicon-rust-plain me-2"></i>Rust
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link d-flex align-items-center" style="color: blue; font-weight: bold; text-decoration: none;" href="https://nnamdi.quarto.pub/finalytics/" target="_blank">
                                <i class="devicon-python-plain me-2"></i>Python
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link d-flex align-items-center" style="color: blue; font-weight: bold; text-decoration: none;" href="/ticker" target="_blank">
                                <i class="bi bi-graph-up me-2"></i>Ticker
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link d-flex align-items-center" style="color: blue; font-weight: bold; text-decoration: none;" href="/portfolio" target="_blank">
                                <i class="bi bi-pie-chart me-2"></i>Portfolio
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
        `;
    }
}

// Register the custom element
customElements.define('navbar-component', NavbarComponent);
