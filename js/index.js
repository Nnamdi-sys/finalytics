import { Ticker, TickerBuilder } from './ticker.js';
import { Tickers, TickersBuilder } from './tickers.js';
import { Portfolio, PortfolioBuilder } from './portfolio.js';
import { Screener } from './screener.js';
import { Chart } from './utils.js'
import Polars from 'nodejs-polars';
import fs from 'fs';
import { fileURLToPath } from 'url';

// Export all classes for use in other modules
export {
  TickerBuilder,
  TickersBuilder,
  PortfolioBuilder,
  Ticker,
  Tickers,
  Portfolio,
  Screener,
  Chart,
};

// Test all methods of the Ticker class
async function testTicker() {
  console.log('=== Testing Ticker ===');
  let ticker;
  try {
    ticker = await new TickerBuilder()
      .symbol('AAPL')
      .startDate('2023-01-01')
      .endDate('2023-12-31')
      .interval('1d')
      .benchmarkSymbol('^GSPC')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .build();
  } catch (err) {
    console.error('Error creating Ticker:', err.message);
    return;
  }

  try {
    const quote = await ticker.getQuote();
    console.log('getQuote:', quote);
  } catch (err) {
    console.error('Error in getQuote:', err.message);
  }

  try {
    const summary = await ticker.getSummaryStats();
    console.log('getSummaryStats:', summary);
  } catch (err) {
    console.error('Error in getSummaryStats:', err.message);
  }

  try {
    const history = await ticker.getPriceHistory();
    console.log('getPriceHistory:', history);
  } catch (err) {
    console.error('Error in getPriceHistory:', err.message);
  }

  try {
    const options = await ticker.getOptionsChain();
    console.log('getOptionsChain:', options);
  } catch (err) {
    console.error('Error in getOptionsChain:', err.message);
  }

  try {
    const news = await ticker.getNews();
    console.log('getNews:', news);
  } catch (err) {
    console.error('Error in getNews:', err.message);
  }

  try {
    const income = await ticker.getIncomeStatement('quarterly', true);
    console.log('getIncomeStatement:', income);
  } catch (err) {
    console.error('Error in getIncomeStatement:', err.message);
  }

  try {
    const balance = await ticker.getBalanceSheet('quarterly', true);
    console.log('getBalanceSheet:', balance);
  } catch (err) {
    console.error('Error in getBalanceSheet:', err.message);
  }

  try {
    const cashflow = await ticker.getCashflowStatement('quarterly', true);
    console.log('getCashflowStatement:', cashflow);
  } catch (err) {
    console.error('Error in getCashflowStatement:', err.message);
  }

  try {
    const ratios = await ticker.getFinancialRatios('quarterly');
    console.log('getFinancialRatios:', ratios);
  } catch (err) {
    console.error('Error in getFinancialRatios:', err.message);
  }

  try {
    const volSurface = await ticker.volatilitySurface();
    console.log('volatilitySurface:', volSurface);
  } catch (err) {
    console.error('Error in volatilitySurface:', err.message);
  }

  try {
    const perfStats = await ticker.performanceStats();
    console.log('performanceStats:', perfStats);
  } catch (err) {
    console.error('Error in performanceStats:', err.message);
  }

  try {
    const perfChart = await ticker.performanceChart(0, 0);
    console.log('performanceChart: Opening in browser...');
    await perfChart.show();
  } catch (err) {
    console.error('Error in performanceChart:', err.message);
  }

  try {
    const candleChart = await ticker.candlestickChart(0, 0);
    console.log('candlestickChart: Opening in browser...');
    await candleChart.show();
  } catch (err) {
    console.error('Error in candlestickChart:', err.message);
  }

  try {
    const optChart = await ticker.optionsChart('surface', 0, 0);
    console.log('optionsChart: Opening in browser...');
    await optChart.show();
  } catch (err) {
    console.error('Error in optionsChart:', err.message);
  }

  try {
    const newsChart = await ticker.newsSentimentChart(0, 0);
    console.log('newsSentimentChart: Opening in browser...');
    await newsChart.show();
  } catch (err) {
    console.error('Error in newsSentimentChart:', err.message);
  }

  try {
    const report = await ticker.report('full');
    console.log('report: Opening in browser...');
    await report.show();
  } catch (err) {
    console.error('Error in report:', err.message);
  }

  ticker.free();
}

// Test all methods of the Tickers class
async function testTickers() {
  console.log('=== Testing Tickers ===');
  let tickers;
  try {
    tickers = await new TickersBuilder()
      .symbols(['AAPL', 'MSFT'])
      .startDate('2023-01-01')
      .endDate('2023-12-31')
      .interval('1d')
      .benchmarkSymbol('^GSPC')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .build();
  } catch (err) {
    console.error('Error creating Tickers:', err.message);
    return;
  }

  try {
    const summary = await tickers.getSummaryStats();
    console.log('getSummaryStats:', summary);
  } catch (err) {
    console.error('Error in getSummaryStats:', err.message);
  }

  try {
    const history = await tickers.getPriceHistory();
    console.log('getPriceHistory:', history);
  } catch (err) {
    console.error('Error in getPriceHistory:', err.message);
  }

  try {
    const options = await tickers.getOptionsChain();
    console.log('getOptionsChain:', options);
  } catch (err) {
    console.error('Error in getOptionsChain:', err.message);
  }

  try {
    const income = await tickers.getIncomeStatement('quarterly', true);
    console.log('getIncomeStatement:', income);
  } catch (err) {
    console.error('Error in getIncomeStatement:', err.message);
  }

  try {
    const balance = await tickers.getBalanceSheet('quarterly', true);
    console.log('getBalanceSheet:', balance);
  } catch (err) {
    console.error('Error in getBalanceSheet:', err.message);
  }

  try {
    const cashflow = await tickers.getCashflowStatement('quarterly', true);
    console.log('getCashflowStatement:', cashflow);
  } catch (err) {
    console.error('Error in getCashflowStatement:', err.message);
  }

  try {
    const ratios = await tickers.getFinancialRatios('quarterly');
    console.log('getFinancialRatios:', ratios);
  } catch (err) {
    console.error('Error in getFinancialRatios:', err.message);
  }

  try {
    const returns = await tickers.returns();
    console.log('returns:', returns);
  } catch (err) {
    console.error('Error in returns:', err.message);
  }

  try {
    const perfStats = await tickers.performanceStats();
    console.log('performanceStats:', perfStats);
  } catch (err) {
    console.error('Error in performanceStats:', err.message);
  }

  try {
    const retChart = await tickers.returnsChart(0, 0);
    console.log('returnsChart: Opening in browser...');
    await retChart.show();
  } catch (err) {
    console.error('Error in returnsChart:', err.message);
  }

  try {
    const retMatrix = await tickers.returnsMatrix(0, 0);
    console.log('returnsMatrix: Opening in browser...');
    await retMatrix.show();
  } catch (err) {
    console.error('Error in returnsMatrix:', err.message);
  }

  try {
    const report = await tickers.report('performance');
    console.log('report: Opening in browser...');
    await report.show();
  } catch (err) {
    console.error('Error in report:', err.message);
  }

  try {
    const ticker = await tickers.getTicker('AAPL');
    console.log('getTicker: Successfully retrieved ticker for AAPL');
    ticker.free();
  } catch (err) {
    console.error('Error in getTicker:', err.message);
  }

  try {
    const portfolio = await tickers.optimize('max_sharpe', '{}', '{}', '{}');
    console.log('optimize: Successfully created portfolio');
    portfolio.free();
  } catch (err) {
    console.error('Error in optimize:', err.message);
  }

  tickers.free();
}

// Test all methods of the Portfolio class
async function testPortfolio() {
  console.log('=== Testing Portfolio ===');
  let portfolio;
  try {
    portfolio = await new PortfolioBuilder()
      .tickerSymbols(['AAPL', 'MSFT', 'NVDA', 'BTC-USD'])
      .benchmarkSymbol('^GSPC')
      .startDate('2023-01-01')
      .endDate('2023-12-31')
      .interval('1d')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .objectiveFunction('max_sharpe')
      .assetConstraints('{}')
      .categoricalConstraints('{}')
      .weights('{}')
      .build();
  } catch (err) {
    console.error('Error creating Portfolio:', err.message);
    return;
  }

  try {
    const results = await portfolio.optimizationResults();
    console.log('optimizationResults:', results);
  } catch (err) {
    console.error('Error in optimizationResults:', err.message);
  }

  try {
    const optChart = await portfolio.optimizationChart(0, 0);
    console.log('optimizationChart: Opening in browser...');
    await optChart.show();
  } catch (err) {
    console.error('Error in optimizationChart:', err.message);
  }

  try {
    const perfChart = await portfolio.performanceChart(0, 0);
    console.log('performanceChart: Opening in browser...');
    await perfChart.show();
  } catch (err) {
    console.error('Error in performanceChart:', err.message);
  }

  try {
    const assetChart = await portfolio.assetReturnsChart(0, 0);
    console.log('assetReturnsChart: Opening in browser...');
    await assetChart.show();
  } catch (err) {
    console.error('Error in assetReturnsChart:', err.message);
  }

  try {
    const retMatrix = await portfolio.returnsMatrix(0, 0);
    console.log('returnsMatrix: Opening in browser...');
    await retMatrix.show();
  } catch (err) {
    console.error('Error in returnsMatrix:', err.message);
  }

  try {
    const report = await portfolio.report('performance');
    console.log('report: Opening in browser...');
    await report.show();
  } catch (err) {
    console.error('Error in report:', err.message);
  }

  portfolio.free();
}

// Test all methods of the Screener class
async function testScreener() {
  console.log('=== Testing Screener ===');
  let screener;
  try {
    screener = await Screener.new(
      'EQUITY',
      [
        JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] }),
        JSON.stringify({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] }),
      ],
      'intradaymarketcap',
      true,
      0,
      10
    );
  } catch (err) {
    console.error('Error creating Screener:', err.message);
    return;
  }

  try {
    const symbols = await screener.symbols();
    console.log('symbols:', symbols);
  } catch (err) {
    console.error('Error in symbols:', err.message);
  }

  try {
    const overview = await screener.overview();
    console.log('overview:', overview);
  } catch (err) {
    console.error('Error in overview:', err.message);
  }

  try {
    const metrics = await screener.metrics();
    console.log('Screener Metrics:', metrics);
  } catch (err) {
    console.error('Error in Screener Metrics:', err.message);
  }

  screener.free();
}

// Test Ticker, Tickers, and Portfolio with CSV data
async function ioTest() {
  console.log('=== IO Test ===');

  const files = {
    nvda: '../examples/datasets/nvda.csv',
    goog: '../examples/datasets/goog.csv',
    aapl: '../examples/datasets/aapl.csv',
    msft: '../examples/datasets/msft.csv',
    btcusd: '../examples/datasets/btcusd.csv',
    gspc: '../examples/datasets/gspc.csv',
  };

  const dataFrames = {};
  for (const [name, path] of Object.entries(files)) {
    try {
      const csvData = fs.readFileSync(path, 'utf8');
      dataFrames[name] = Polars.readCSV(csvData);
    } catch (err) {
      console.error(`Error reading ${path}:`, err.message);
      return;
    }
  }

  console.log('--- Testing Ticker ---');
  let ticker;
  try {
    ticker = await new TickerBuilder()
      .symbol('AAPL')
      .startDate('2023-01-01')
      .endDate('2023-12-31')
      .interval('1d')
      .benchmarkSymbol('^GSPC')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .tickerData(dataFrames['aapl'])
      .benchmarkData(dataFrames['gspc'])
      .build();
    const report = await ticker.report('performance');
    console.log('Ticker report: Opening in browser...');
    await report.show();
  } catch (err) {
    console.error('Error in Ticker report:', err.message);
  } finally {
    if (ticker) ticker.free();
  }

  console.log('--- Testing Tickers ---');
  let tickers;
  try {
    const tickersData = [
      dataFrames['nvda'],
      dataFrames['goog'],
      dataFrames['aapl'],
      dataFrames['msft'],
      dataFrames['btcusd'],
    ];
    tickers = await new TickersBuilder()
      .symbols(['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'])
      .startDate('2023-01-01')
      .endDate('2023-12-31')
      .interval('1d')
      .benchmarkSymbol('^GSPC')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .tickersData(tickersData)
      .benchmarkData(dataFrames['gspc'])
      .build();
    const report = await tickers.report('full');
    console.log('Tickers report: Opening in browser...');
    await report.show();
  } catch (err) {
    console.error('Error in Tickers report:', err.message);
  } finally {
    if (tickers) tickers.free();
  }

  console.log('--- Testing Portfolio ---');
  let portfolio;
  try {
    const tickersData = [
      dataFrames['nvda'],
      dataFrames['goog'],
      dataFrames['aapl'],
      dataFrames['msft'],
      dataFrames['btcusd'],
    ];
    const assetConstraints = JSON.stringify([
      [0, 1],
      [0, 1],
      [0, 1],
      [0, 1],
      [0, 1],
    ]);
    const categoricalConstraints = JSON.stringify([
      {
        Name: 'AssetClass',
        Categories: ['EQUITY', 'EQUITY', 'EQUITY', 'EQUITY', 'CRYPTO'],
        Constraints: [
          ['EQUITY', 0.0, 0.8],
          ['CRYPTO', 0.0, 0.2],
        ],
      },
    ]);
    portfolio = await new PortfolioBuilder()
      .tickerSymbols(['NVDA', 'GOOG', 'AAPL', 'MSFT', 'BTC-USD'])
      .benchmarkSymbol('^GSPC')
      .startDate('2023-01-01')
      .endDate('2023-12-31')
      .interval('1d')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .objectiveFunction('max_sharpe')
      .assetConstraints(assetConstraints)
      .categoricalConstraints(categoricalConstraints)
      .weights('{}')
      .tickersData(tickersData)
      .benchmarkData(dataFrames['gspc'])
      .build();
    const report = await portfolio.report('full');
    console.log('Portfolio report: Opening in browser...');
    await report.show();
  } catch (err) {
    console.error('Error in Portfolio report:', err.message);
  } finally {
    if (portfolio) portfolio.free();
  }
}

// README example demonstrating a practical workflow
async function readmeExample() {
  console.log('=== README Example ===');

  let screener;
  try {
    screener = await Screener.new(
      'EQUITY',
      [
        JSON.stringify({ operator: 'eq', operands: ['exchange', 'NMS'] }),
        JSON.stringify({ operator: 'gte', operands: ['intradaymarketcap', 10000000000] }),
      ],
      'intradaymarketcap',
      true,
      0,
      10
    );
  } catch (err) {
    console.error('Error creating Screener:', err.message);
    return;
  }

  let symbols;
  try {
    symbols = await screener.symbols();
    console.log('Screened Symbols:', symbols);
  } catch (err) {
    console.error('Failed to get symbols:', err.message);
    screener.free();
    return;
  }

  let tickers;
  try {
    tickers = await new TickersBuilder()
      .symbols(symbols)
      .startDate('2023-01-01')
      .endDate('2024-12-31')
      .interval('1d')
      .benchmarkSymbol('^GSPC')
      .confidenceLevel(0.95)
      .riskFreeRate(0.02)
      .build();
  } catch (err) {
    console.error('Failed to create Tickers:', err.message);
    screener.free();
    return;
  }

  if (symbols.length > 0) {
    let ticker;
    try {
      ticker = await tickers.getTicker(symbols[0]);
      for (const reportType of ['performance', 'financials', 'options', 'news']) {
        try {
          const report = await ticker.report(reportType);
          console.log(`Ticker ${reportType} report: Opening in browser...`);
          await report.show();
        } catch (err) {
          console.error(`Failed to get ${reportType} report:`, err.message);
        }
      }
    } catch (err) {
      console.error('Failed to get Ticker:', err.message);
    } finally {
      if (ticker) ticker.free();
    }
  }

  try {
    const tickersReport = await tickers.report('performance');
    console.log('Tickers report: Opening in browser...');
    await tickersReport.show();
  } catch (err) {
    console.error('Failed to get Tickers report:', err.message);
  }

  let portfolio;
  try {
    portfolio = await tickers.optimize('max_sharpe', '{}', '{}', '{}');
    const portfolioReport = await portfolio.report('performance');
    console.log('Portfolio report: Opening in browser...');
    await portfolioReport.show();
  } catch (err) {
    console.error('Failed to optimize portfolio or get report:', err.message);
  } finally {
    if (portfolio) portfolio.free();
  }

  tickers.free();
  screener.free();
}

// Run all tests
async function main() {
  await testTicker();
  await testTickers();
  await testPortfolio();
  await testScreener();
  await ioTest();
  await readmeExample();
}

if (fileURLToPath(import.meta.url) === process.argv[1]) {
  main().catch(err => console.error('Main error:', err.message));
}