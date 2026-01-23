/**
 * NowNodes Mainnet Authentication Proxy
 * 
 * This proxy adds the required api-key header for NowNodes API access.
 * Required for Odra contract deployment to mainnet via NowNodes RPC.
 * 
 * Usage:
 * 1. Run: node nownodes_mainnet_proxy.js
 * 2. Keep running during deployment
 * 3. Deploy: cargo run --bin deploy_mainnet --features=livenet
 */

const http = require('http');
const https = require('https');
const fs = require('fs');
const path = require('path');

function loadEnv() {
  try {
    const envPath = path.join(__dirname, '.env');
    const envContent = fs.readFileSync(envPath, 'utf8');
    
    envContent.split('\n').forEach(line => {
      const trimmed = line.trim();
      if (trimmed && !trimmed.startsWith('#')) {
        const [key, ...values] = trimmed.split('=');
        if (key && values.length > 0) {
          process.env[key.trim()] = values.join('=').trim();
        }
      }
    });
  } catch (error) {
    console.error('Error: Could not read .env file');
    process.exit(1);
  }
}

loadEnv();

const NOWNODES_API_KEY = process.env.NOWNODES_API_KEY;
const NOWNODES_URL = 'https://casper.nownodes.io';
const LOCAL_PORT = 7778;

if (!NOWNODES_API_KEY) {
  console.error('Error: NOWNODES_API_KEY not found in .env file');
  process.exit(1);
}

// Create proxy server
const server = http.createServer((req, res) => {
  let body = '';
  
  // Collect request body
  req.on('data', chunk => {
    body += chunk.toString();
  });
  
  req.on('end', () => {
    let targetPath = req.url;
    console.log(`Incoming request: ${req.method} ${req.url}`);
    

    const options = {
      hostname: 'casper.nownodes.io',
      port: 443,
      path: targetPath,
      method: req.method,
      headers: {
        'Content-Type': 'application/json',
        'api-key': NOWNODES_API_KEY,
        'Content-Length': Buffer.byteLength(body)
      }
    };
    
    console.log(`Forwarding to: https://casper.nownodes.io${targetPath}`);
    
    const proxyReq = https.request(options, (proxyRes) => {
      // Forward response headers
      res.writeHead(proxyRes.statusCode, proxyRes.headers);
      
      // Forward response body
      proxyRes.on('data', chunk => {
        res.write(chunk);
      });
      
      proxyRes.on('end', () => {
        res.end();
      });
    });
    
    proxyReq.on('error', (error) => {
      console.error('Proxy error:', error.message);
      res.writeHead(500, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ error: 'Proxy error', details: error.message }));
    });
    
    // Send request body
    if (body) {
      proxyReq.write(body);
    }
    
    proxyReq.end();
  });
});

// Start proxy server
server.listen(LOCAL_PORT, '127.0.0.1', () => {
  console.log('NowNodes Mainnet Proxy running on http://127.0.0.1:7778');
  console.log('Forwarding to:', NOWNODES_URL);
  console.log('Using API Key:', NOWNODES_API_KEY.substring(0, 20) + '...');
  console.log('Ready for mainnet deployment!');
  console.log('Run: cargo run --bin deploy_mainnet --features=livenet');
  console.log('');
  console.log('Press Ctrl+C to stop');
});
