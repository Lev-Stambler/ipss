// Runs a node on a random UDP port that attempts to collect 10 peers for an
// infohash, then keeps running as a passive DHT node.
//
// IMPORTANT: if the UDP port is not reachable from the public internet, you
// may see very few results.
//
// To collect 10 peers, it usually has to contact some 1k nodes. It's much easier
// to find peers for popular infohashes. This process is not instant and should
// take a minute or two, depending on your network connection.
//
//
// There is a builtin web server that can be used to collect debugging stats
// from http://localhost:8711/debug/vars.
package main

import (
	"sync"

	"github.com/Lev-Stambler/ipss/ipss-node/src/init"
)

func main() {
	wg := new(sync.WaitGroup)
	wg.Add(1)
	go init.StartDHT(wg)
	exit(0)
}
