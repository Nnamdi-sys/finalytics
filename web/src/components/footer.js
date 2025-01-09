// footer-component.js
class FooterComponent extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.innerHTML = `
        <footer class="footer bg-dark text-light">
            <div class="container">
                <div class="row align-items-center justify-content-between">
                    <div class="col-md-auto">
                        <a class="navbar-brand" href="#">
                            <img src="../images/logo.svg" width="200" height="50" class="d-inline-block align-top ml-0" alt="Logo" style="filter: invert(100%);">
                        </a>
                        <p class="mt-3 mb-4 text-sm text-gray-500">A Rust and Python library for financial data analysis.</p>
                    </div>
                    <div class="col-md-auto">
                        <div class="pt-4"></div>
                        <h5>Rust Documentation</h5>
                        <ul class="list-unstyled">
                            <li>
                                <a href="https://crates.io/crates/finalytics" target="_blank">
                                    <i class="bi bi-box text-light me-2"></i>Crates.io
                                </a>
                            </li>
                            <li>
                                <a href="https://docs.rs/finalytics/latest/finalytics/" target="_blank">
                                    <i class="bi bi-file-text text-light me-2"></i>Docs.rs
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div class="col-md-auto">
                        <div class="pt-4"></div>
                        <h5>Python Documentation</h5>
                        <ul class="list-unstyled">
                            <li>
                                <a href="https://pypi.org/project/finalytics/" target="_blank">
                                    <i class="bi bi-box text-light me-2"></i>PyPi
                                </a>
                            </li>
                            <li>
                                <a href="https://nnamdi.quarto.pub/finalytics/" target="_blank">
                                    <i class="bi bi-file-text text-light me-2"></i>Quarto
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div class="col-md-auto">
                        <div class="pt-4"></div>
                        <h5>Dashboards</h5>
                        <ul class="list-unstyled">
                            <li>
                                <a href="/ticker" target="_blank">
                                    <i class="bi bi-graph-up text-light me-2"></i>Ticker
                                </a>
                            </li>
                            <li>
                                <a href="/portfolio" target="_blank">
                                    <i class="bi bi-pie-chart text-light me-2"></i>Portfolio
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <div class="container mx-auto py-4 px-5 flex flex-wrap flex-col sm:flex-row">
                <p class="text-gray-600 text-sm text-center sm:text-left font-poppins">
                    © 2024 Finalytics —
                    <a class="text-gray-700 ml-1" href="https://twitter.com/finalytics_rs" target="_blank">@finalytics_rs</a>
                </p>
            </div>
        </footer>
        `;
    }
}

// Register the custom element
customElements.define('footer-component', FooterComponent);
