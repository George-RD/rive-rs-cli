const HERO_ARTIFACT = "examples/authoring/complex-animated-showcase.v0.riv";

function mountHero() {
  const parcel = document.querySelector(".proof-parcel");
  const canvas = document.querySelector(".hero-scene");
  if (!parcel || !canvas || !window.RivePlayback) return;

  parcel.dataset.artifact = HERO_ARTIFACT;
  parcel.dataset.playbackReady = "false";
  parcel.dataset.playing = "false";

  const timeline = RivePlayback.createTimeline({
    canvas,
    src: HERO_ARTIFACT,
    autoplay: true,
    onPlayingChange(playing) {
      parcel.dataset.playing = String(playing);
    },
  });

  timeline.ready
    .then(() => {
      parcel.dataset.playbackReady = "true";
    })
    .catch((error) => {
      parcel.dataset.playbackReady = "error";
      console.error("could not start landing proof", error);
    });

  window.addEventListener("resize", () => timeline.resize());
  window.addEventListener("pagehide", (event) => {
    if (!event.persisted) timeline.destroy();
  });
}

mountHero();
