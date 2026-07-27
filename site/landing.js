function mountHero() {
  if (!window.rive) return;
  rive.RuntimeLoader.setWasmUrl("assets/rive.wasm");
  const canvas = document.querySelector(".hero-scene");
  if (!canvas) return;
  const instance = new rive.Rive({
    canvas,
    src: "parity/official/coffee_loader.riv",
    stateMachines: ["State Machine 1"],
    autoplay: true,
    fit: rive.Fit.contain,
    alignment: rive.Alignment.center,
  });
  instance.on(rive.EventType.Load, () => instance.resizeDrawingSurfaceToCanvas());
}

mountHero();
