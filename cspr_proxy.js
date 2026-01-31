/**
 * CSPR.cloud Authentication Proxy
 * 
 * This proxy adds the required Authorization header for CSPR.cloud API access.
 * Required for Odra contract deployment since it doesn't support custom headers natively.
 * 
 * Usage:
 * 1. Run: node cspr_proxy.js
 * 2. Keep running during deployment
 * 3. Deploy: cargo run --bin deploy_testnet --features=livenet
 */

const http = require('http');
const https = require('https');
const fs = require('fs');
const path = require('path');

// Read .env file
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
    console.error('❌ Error: Could not read .env file');
    process.exit(1);
  }
}

loadEnv();

// CSPR.cloud configuration from environment
const CSPR_CLOUD_TOKEN = process.env.CSPR_CLOUD_AUTH_TOKEN;
const CSPR_CLOUD_URL = 'https://node.testnet.cspr.cloud';
const LOCAL_PORT = 7778;

if (!CSPR_CLOUD_TOKEN) {
  console.error('❌ Error: CSPR_CLOUD_AUTH_TOKEN not found in .env file');
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
    // Forward request to CSPR.cloud with authentication
    const options = {
      hostname: 'node.testnet.cspr.cloud',
      port: 443,
      path: req.url,
      method: req.method,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': CSPR_CLOUD_TOKEN,
        'Content-Length': Buffer.byteLength(body)
      }
    };
    
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
      console.error('❌ Proxy error:', error.message);
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
  console.log('🚀 CSPR.cloud Proxy running on http://127.0.0.1:7778');
  console.log('📡 Forwarding to:', CSPR_CLOUD_URL);
  console.log('');
  console.log('✅ Ready for contract deployment!');
  console.log('   Run: cargo run --bin deploy_testnet --features=livenet');
  console.log('');
  console.log('Press Ctrl+C to stop');
});
