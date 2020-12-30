package main

import (
	"fmt"
	"github.com/bytecodealliance/wasmtime-go"
)

func main() {
	engine := wasmtime.NewEngine()
	store := wasmtime.NewStore(engine)

	linker := wasmtime.NewLinker(store)

	module, err := wasmtime.NewModuleFromFile(engine, "../target/wasm32-wasi/release/loki_bench.wasm")
	check(err)

	for _, v := range module.Imports() {
		fmt.Printf("%v ", v.Name())
	}

	wasiConfig := wasmtime.NewWasiConfig()
	wasiConfig.InheritEnv()
	wasiConfig.InheritStderr()
	wasiConfig.InheritStdout()
	err = wasiConfig.PreopenDir(".", ".")
	check(err)
	err = wasiConfig.PreopenDir("./data", "./data")
	check(err)
	wasi, err := wasmtime.NewWasiInstance(store, wasiConfig, "wasi_snapshot_preview1")
	check(err)

	err = linker.DefineWasi(wasi)
	check(err)

	instance, err := linker.Instantiate(module)
	check(err)

	nom := instance.GetExport("run_nom").Func()
	_, err = nom.Call()
	check(err)
}

func check(err error) {
	if err != nil {
        panic(err)
	}
}
