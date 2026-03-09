(function webpackUniversalModuleDefinition(root, factory) {
	if(typeof exports === 'object' && typeof module === 'object')
		module.exports = factory();
	else if(typeof define === 'function' && define.amd)
		define([], factory);
	else if(typeof exports === 'object')
		exports["rive"] = factory();
	else
		root["rive"] = factory();
})(this, () => {
return /******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ([
/* 0 */,
/* 1 */
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });

var Rive = (() => {
  var _scriptName = typeof document != 'undefined' ? document.currentScript?.src : undefined;
  
  return (
function(moduleArg = {}) {
  var moduleRtn;

var m = moduleArg, ba, ca, da = new Promise((a, b) => {
  ba = a;
  ca = b;
}), ea = "object" == typeof window, fa = "function" == typeof importScripts;
function ha() {
  function a(g) {
    const k = d;
    c = b = 0;
    d = new Map();
    k.forEach(p => {
      try {
        p(g);
      } catch (n) {
        console.error(n);
      }
    });
    this.ob();
    e && e.Qb();
  }
  let b = 0, c = 0, d = new Map(), e = null, f = null;
  this.requestAnimationFrame = function(g) {
    b ||= requestAnimationFrame(a.bind(this));
    const k = ++c;
    d.set(k, g);
    return k;
  };
  this.cancelAnimationFrame = function(g) {
    d.delete(g);
    b && 0 == d.size && (cancelAnimationFrame(b), b = 0);
  };
  this.Ob = function(g) {
    f && (document.body.remove(f), f = null);
    g || (f = document.createElement("div"), f.style.backgroundColor = "black", f.style.position = "fixed", f.style.right = 0, f.style.top = 0, f.style.color = "white", f.style.padding = "4px", f.innerHTML = "RIVE FPS", g = function(k) {
      f.innerHTML = "RIVE FPS " + k.toFixed(1);
    }, document.body.appendChild(f));
    e = new function() {
      let k = 0, p = 0;
      this.Qb = function() {
        var n = performance.now();
        p ? (++k, n -= p, 1000 < n && (g(1000 * k / n), k = p = 0)) : (p = n, k = 0);
      };
    }();
  };
  this.Lb = function() {
    f && (document.body.remove(f), f = null);
    e = null;
  };
  this.ob = function() {
  };
}
function ia(a) {
  console.assert(!0);
  const b = new Map();
  let c = -Infinity;
  this.push = function(d) {
    d = d + ((1 << a) - 1) >> a;
    b.has(d) && clearTimeout(b.get(d));
    b.set(d, setTimeout(function() {
      b.delete(d);
      0 == b.length ? c = -Infinity : d == c && (c = Math.max(...b.keys()), console.assert(c < d));
    }, 1000));
    c = Math.max(d, c);
    return c << a;
  };
}
const ja = m.onRuntimeInitialized;
m.onRuntimeInitialized = function() {
  ja && ja();
  let a = m.decodeAudio;
  m.decodeAudio = function(e, f) {
    e = a(e);
    f(e);
  };
  let b = m.decodeFont;
  m.decodeFont = function(e, f) {
    e = b(e);
    f(e);
  };
  const c = m.FileAssetLoader;
  m.ptrToAsset = e => {
    let f = m.ptrToFileAsset(e);
    return f.isImage ? m.ptrToImageAsset(e) : f.isFont ? m.ptrToFontAsset(e) : f.isAudio ? m.ptrToAudioAsset(e) : f;
  };
  m.CustomFileAssetLoader = c.extend("CustomFileAssetLoader", {__construct:function({loadContents:e}) {
    this.__parent.__construct.call(this);
    this.Eb = e;
  }, loadContents:function(e, f) {
    e = m.ptrToAsset(e);
    return this.Eb(e, f);
  },});
  m.CDNFileAssetLoader = c.extend("CDNFileAssetLoader", {__construct:function() {
    this.__parent.__construct.call(this);
  }, loadContents:function(e) {
    let f = m.ptrToAsset(e);
    e = f.cdnUuid;
    if ("" === e) {
      return !1;
    }
    (function(g, k) {
      var p = new XMLHttpRequest();
      p.responseType = "arraybuffer";
      p.onreadystatechange = function() {
        4 == p.readyState && 200 == p.status && k(p);
      };
      p.open("GET", g, !0);
      p.send(null);
    })(f.cdnBaseUrl + "/" + e, g => {
      f.decode(new Uint8Array(g.response));
    });
    return !0;
  },});
  m.FallbackFileAssetLoader = c.extend("FallbackFileAssetLoader", {__construct:function() {
    this.__parent.__construct.call(this);
    this.kb = [];
  }, addLoader:function(e) {
    this.kb.push(e);
  }, loadContents:function(e, f) {
    for (let g of this.kb) {
      if (g.loadContents(e, f)) {
        return !0;
      }
    }
    return !1;
  },});
  let d = m.computeAlignment;
  m.computeAlignment = function(e, f, g, k, p = 1.0) {
    return d.call(this, e, f, g, k, p);
  };
};
const ka = "createConicGradient createImageData createLinearGradient createPattern createRadialGradient getContextAttributes getImageData getLineDash getTransform isContextLost isPointInPath isPointInStroke measureText".split(" "), la = new function() {
  function a() {
    if (!b) {
      var l = document.createElement("canvas"), u = {alpha:1, depth:0, stencil:0, antialias:0, premultipliedAlpha:1, preserveDrawingBuffer:0, powerPreference:"high-performance", failIfMajorPerformanceCaveat:0, enableExtensionsByDefault:1, explicitSwapControl:1, renderViaOffscreenBackBuffer:1,};
      let r;
      if (/iPhone|iPad|iPod/i.test(navigator.userAgent)) {
        if (r = l.getContext("webgl", u), c = 1, !r) {
          return console.log("No WebGL support. Image mesh will not be drawn."), !1;
        }
      } else {
        if (r = l.getContext("webgl2", u)) {
          c = 2;
        } else {
          if (r = l.getContext("webgl", u)) {
            c = 1;
          } else {
            return console.log("No WebGL support. Image mesh will not be drawn."), !1;
          }
        }
      }
      r = new Proxy(r, {get(G, w) {
        if (G.isContextLost()) {
          if (p || (console.error("Cannot render the mesh because the GL Context was lost. Tried to invoke ", w), p = !0), "function" === typeof G[w]) {
            return function() {
            };
          }
        } else {
          return "function" === typeof G[w] ? function(...M) {
            return G[w].apply(G, M);
          } : G[w];
        }
      }, set(G, w, M) {
        if (G.isContextLost()) {
          p || (console.error("Cannot render the mesh because the GL Context was lost. Tried to set property " + w), p = !0);
        } else {
          return G[w] = M, !0;
        }
      },});
      d = Math.min(r.getParameter(r.MAX_RENDERBUFFER_SIZE), r.getParameter(r.MAX_TEXTURE_SIZE));
      function C(G, w, M) {
        w = r.createShader(w);
        r.shaderSource(w, M);
        r.compileShader(w);
        M = r.getShaderInfoLog(w);
        if (0 < (M || "").length) {
          throw M;
        }
        r.attachShader(G, w);
      }
      l = r.createProgram();
      C(l, r.VERTEX_SHADER, "attribute vec2 vertex;\n                attribute vec2 uv;\n                uniform vec4 mat;\n                uniform vec2 translate;\n                varying vec2 st;\n                void main() {\n                    st = uv;\n                    gl_Position = vec4(mat2(mat) * vertex + translate, 0, 1);\n                }");
      C(l, r.FRAGMENT_SHADER, "precision highp float;\n                uniform sampler2D image;\n                varying vec2 st;\n                void main() {\n                    gl_FragColor = texture2D(image, st);\n                }");
      r.bindAttribLocation(l, 0, "vertex");
      r.bindAttribLocation(l, 1, "uv");
      r.linkProgram(l);
      u = r.getProgramInfoLog(l);
      if (0 < (u || "").trim().length) {
        throw u;
      }
      e = r.getUniformLocation(l, "mat");
      f = r.getUniformLocation(l, "translate");
      r.useProgram(l);
      r.bindBuffer(r.ARRAY_BUFFER, r.createBuffer());
      r.enableVertexAttribArray(0);
      r.enableVertexAttribArray(1);
      r.bindBuffer(r.ELEMENT_ARRAY_BUFFER, r.createBuffer());
      r.uniform1i(r.getUniformLocation(l, "image"), 0);
      r.pixelStorei(r.UNPACK_PREMULTIPLY_ALPHA_WEBGL, !0);
      b = r;
    }
    return !0;
  }
  let b = null, c = 0, d = 0, e = null, f = null, g = 0, k = 0, p = !1;
  a();
  this.bc = function() {
    a();
    return d;
  };
  this.Kb = function(l) {
    b.deleteTexture && b.deleteTexture(l);
  };
  this.Jb = function(l) {
    if (!a()) {
      return null;
    }
    const u = b.createTexture();
    if (!u) {
      return null;
    }
    b.bindTexture(b.TEXTURE_2D, u);
    b.texImage2D(b.TEXTURE_2D, 0, b.RGBA, b.RGBA, b.UNSIGNED_BYTE, l);
    b.texParameteri(b.TEXTURE_2D, b.TEXTURE_WRAP_S, b.CLAMP_TO_EDGE);
    b.texParameteri(b.TEXTURE_2D, b.TEXTURE_WRAP_T, b.CLAMP_TO_EDGE);
    b.texParameteri(b.TEXTURE_2D, b.TEXTURE_MAG_FILTER, b.LINEAR);
    2 == c ? (b.texParameteri(b.TEXTURE_2D, b.TEXTURE_MIN_FILTER, b.LINEAR_MIPMAP_LINEAR), b.generateMipmap(b.TEXTURE_2D)) : b.texParameteri(b.TEXTURE_2D, b.TEXTURE_MIN_FILTER, b.LINEAR);
    return u;
  };
  const n = new ia(8), t = new ia(8), x = new ia(10), y = new ia(10);
  this.Nb = function(l, u, r, C, G) {
    if (a()) {
      var w = n.push(l), M = t.push(u);
      if (b.canvas) {
        if (b.canvas.width != w || b.canvas.height != M) {
          b.canvas.width = w, b.canvas.height = M;
        }
        b.viewport(0, M - u, l, u);
        b.disable(b.SCISSOR_TEST);
        b.clearColor(0, 0, 0, 0);
        b.clear(b.COLOR_BUFFER_BIT);
        b.enable(b.SCISSOR_TEST);
        r.sort((J, Z) => Z.vb - J.vb);
        w = x.push(C);
        g != w && (b.bufferData(b.ARRAY_BUFFER, 8 * w, b.DYNAMIC_DRAW), g = w);
        w = 0;
        for (var T of r) {
          b.bufferSubData(b.ARRAY_BUFFER, w, T.Ta), w += 4 * T.Ta.length;
        }
        console.assert(w == 4 * C);
        for (var W of r) {
          b.bufferSubData(b.ARRAY_BUFFER, w, W.Bb), w += 4 * W.Bb.length;
        }
        console.assert(w == 8 * C);
        w = y.push(G);
        k != w && (b.bufferData(b.ELEMENT_ARRAY_BUFFER, 2 * w, b.DYNAMIC_DRAW), k = w);
        T = 0;
        for (var pa of r) {
          b.bufferSubData(b.ELEMENT_ARRAY_BUFFER, T, pa.indices), T += 2 * pa.indices.length;
        }
        console.assert(T == 2 * G);
        pa = 0;
        W = !0;
        w = T = 0;
        for (const J of r) {
          J.image.Ja != pa && (b.bindTexture(b.TEXTURE_2D, J.image.Ia || null), pa = J.image.Ja);
          J.hc ? (b.scissor(J.Za, M - J.$a - J.jb, J.uc, J.jb), W = !0) : W && (b.scissor(0, M - u, l, u), W = !1);
          r = 2 / l;
          const Z = -2 / u;
          b.uniform4f(e, J.ha[0] * r * J.Aa, J.ha[1] * Z * J.Ba, J.ha[2] * r * J.Aa, J.ha[3] * Z * J.Ba);
          b.uniform2f(f, J.ha[4] * r * J.Aa + r * (J.Za - J.cc * J.Aa) - 1, J.ha[5] * Z * J.Ba + Z * (J.$a - J.dc * J.Ba) + 1);
          b.vertexAttribPointer(0, 2, b.FLOAT, !1, 0, w);
          b.vertexAttribPointer(1, 2, b.FLOAT, !1, 0, w + 4 * C);
          b.drawElements(b.TRIANGLES, J.indices.length, b.UNSIGNED_SHORT, T);
          w += 4 * J.Ta.length;
          T += 2 * J.indices.length;
        }
        console.assert(w == 4 * C);
        console.assert(T == 2 * G);
      }
    }
  };
  this.canvas = function() {
    return a() && b.canvas;
  };
}(), ma = m.onRuntimeInitialized;
m.onRuntimeInitialized = function() {
  function a(q) {
    switch(q) {
      case n.srcOver:
        return "source-over";
      case n.screen:
        return "screen";
      case n.overlay:
        return "overlay";
      case n.darken:
        return "darken";
      case n.lighten:
        return "lighten";
      case n.colorDodge:
        return "color-dodge";
      case n.colorBurn:
        return "color-burn";
      case n.hardLight:
        return "hard-light";
      case n.softLight:
        return "soft-light";
      case n.difference:
        return "difference";
      case n.exclusion:
        return "exclusion";
      case n.multiply:
        return "multiply";
      case n.hue:
        return "hue";
      case n.saturation:
        return "saturation";
      case n.color:
        return "color";
      case n.luminosity:
        return "luminosity";
    }
  }
  function b(q) {
    return "rgba(" + ((16711680 & q) >>> 16) + "," + ((65280 & q) >>> 8) + "," + ((255 & q) >>> 0) + "," + ((4278190080 & q) >>> 24) / 255 + ")";
  }
  function c() {
    0 < M.length && (la.Nb(w.drawWidth(), w.drawHeight(), M, T, W), M = [], W = T = 0, w.reset(512, 512));
    for (const q of G) {
      for (const v of q.I) {
        v();
      }
      q.I = [];
    }
    G.clear();
  }
  ma && ma();
  var d = m.RenderPaintStyle;
  const e = m.RenderPath, f = m.RenderPaint, g = m.Renderer, k = m.StrokeCap, p = m.StrokeJoin, n = m.BlendMode, t = d.fill, x = d.stroke, y = m.FillRule.evenOdd;
  let l = 1;
  var u = m.RenderImage.extend("CanvasRenderImage", {__construct:function({la:q, wa:v} = {}) {
    this.__parent.__construct.call(this);
    this.Ja = l;
    l = l + 1 & 2147483647 || 1;
    this.la = q;
    this.wa = v;
  }, __destruct:function() {
    this.Ia && (la.Kb(this.Ia), URL.revokeObjectURL(this.Wa));
    this.__parent.__destruct.call(this);
  }, decode:function(q) {
    var v = this;
    v.wa && v.wa(v);
    var H = new Image();
    v.Wa = URL.createObjectURL(new Blob([q], {type:"image/png",}));
    H.onload = function() {
      v.Db = H;
      v.Ia = la.Jb(H);
      v.size(H.width, H.height);
      v.la && v.la(v);
    };
    H.src = v.Wa;
  },}), r = e.extend("CanvasRenderPath", {__construct:function() {
    this.__parent.__construct.call(this);
    this.U = new Path2D();
  }, rewind:function() {
    this.U = new Path2D();
  }, addPath:function(q, v, H, E, K, I, F) {
    var L = this.U, aa = L.addPath;
    q = q.U;
    const R = new DOMMatrix();
    R.a = v;
    R.b = H;
    R.c = E;
    R.d = K;
    R.e = I;
    R.f = F;
    aa.call(L, q, R);
  }, fillRule:function(q) {
    this.Va = q;
  }, moveTo:function(q, v) {
    this.U.moveTo(q, v);
  }, lineTo:function(q, v) {
    this.U.lineTo(q, v);
  }, cubicTo:function(q, v, H, E, K, I) {
    this.U.bezierCurveTo(q, v, H, E, K, I);
  }, close:function() {
    this.U.closePath();
  },}), C = f.extend("CanvasRenderPaint", {color:function(q) {
    this.Xa = b(q);
  }, thickness:function(q) {
    this.Gb = q;
  }, join:function(q) {
    switch(q) {
      case p.miter:
        this.Ha = "miter";
        break;
      case p.round:
        this.Ha = "round";
        break;
      case p.bevel:
        this.Ha = "bevel";
    }
  }, cap:function(q) {
    switch(q) {
      case k.butt:
        this.Ga = "butt";
        break;
      case k.round:
        this.Ga = "round";
        break;
      case k.square:
        this.Ga = "square";
    }
  }, style:function(q) {
    this.Fb = q;
  }, blendMode:function(q) {
    this.Cb = a(q);
  }, clearGradient:function() {
    this.ja = null;
  }, linearGradient:function(q, v, H, E) {
    this.ja = {xb:q, yb:v, cb:H, eb:E, Qa:[],};
  }, radialGradient:function(q, v, H, E) {
    this.ja = {xb:q, yb:v, cb:H, eb:E, Qa:[], ac:!0,};
  }, addStop:function(q, v) {
    this.ja.Qa.push({color:q, stop:v,});
  }, completeGradient:function() {
  }, draw:function(q, v, H, E) {
    let K = this.Fb;
    var I = this.Xa, F = this.ja;
    const L = q.globalCompositeOperation, aa = q.globalAlpha;
    q.globalCompositeOperation = this.Cb;
    q.globalAlpha = E;
    if (null != F) {
      I = F.xb;
      const X = F.yb, S = F.cb;
      var R = F.eb;
      E = F.Qa;
      F.ac ? (F = S - I, R -= X, I = q.createRadialGradient(I, X, 0, I, X, Math.sqrt(F * F + R * R))) : I = q.createLinearGradient(I, X, S, R);
      for (let xa = 0, Ya = E.length; xa < Ya; xa++) {
        F = E[xa], I.addColorStop(F.stop, b(F.color));
      }
      this.Xa = I;
      this.ja = null;
    }
    switch(K) {
      case x:
        q.strokeStyle = I;
        q.lineWidth = this.Gb;
        q.lineCap = this.Ga;
        q.lineJoin = this.Ha;
        q.stroke(v);
        break;
      case t:
        q.fillStyle = I, q.fill(v, H);
    }
    q.globalCompositeOperation = L;
    q.globalAlpha = aa;
  },});
  const G = new Set();
  let w = null, M = [], T = 0, W = 0;
  var pa = m.CanvasRenderer = g.extend("Renderer", {__construct:function(q) {
    this.__parent.__construct.call(this);
    this.T = [1, 0, 0, 1, 0, 0];
    this.G = [1.0];
    this.B = q.getContext("2d");
    this.Ua = q;
    this.I = [];
  }, save:function() {
    this.T.push(...this.T.slice(this.T.length - 6));
    this.G.push(this.G[this.G.length - 1]);
    this.I.push(this.B.save.bind(this.B));
  }, restore:function() {
    const q = this.T.length - 6;
    if (6 > q) {
      throw "restore() called without matching save().";
    }
    this.T.splice(q);
    this.G.pop();
    this.I.push(this.B.restore.bind(this.B));
  }, transform:function(q, v, H, E, K, I) {
    const F = this.T, L = F.length - 6;
    F.splice(L, 6, F[L] * q + F[L + 2] * v, F[L + 1] * q + F[L + 3] * v, F[L] * H + F[L + 2] * E, F[L + 1] * H + F[L + 3] * E, F[L] * K + F[L + 2] * I + F[L + 4], F[L + 1] * K + F[L + 3] * I + F[L + 5]);
    this.I.push(this.B.transform.bind(this.B, q, v, H, E, K, I));
  }, rotate:function(q) {
    const v = Math.sin(q);
    q = Math.cos(q);
    this.transform(q, v, -v, q, 0, 0);
  }, modulateOpacity:function(q) {
    this.G[this.G.length - 1] *= q;
  }, _drawPath:function(q, v) {
    this.I.push(v.draw.bind(v, this.B, q.U, q.Va === y ? "evenodd" : "nonzero", Math.max(0, this.G[this.G.length - 1])));
  }, _drawRiveImage:function(q, v, H, E) {
    var K = q.Db;
    if (K) {
      var I = this.B, F = a(H), L = Math.max(0, E * this.G[this.G.length - 1]);
      this.I.push(function() {
        I.globalCompositeOperation = F;
        I.globalAlpha = L;
        I.drawImage(K, 0, 0);
        I.globalAlpha = 1;
      });
    }
  }, _getMatrix:function(q) {
    const v = this.T, H = v.length - 6;
    for (let E = 0; 6 > E; ++E) {
      q[E] = v[H + E];
    }
  }, _drawImageMesh:function(q, v, H, E, K, I, F, L, aa, R, X) {
    v = this.B.canvas.width;
    var S = this.B.canvas.height;
    const xa = R - L, Ya = X - aa;
    L = Math.max(L, 0);
    aa = Math.max(aa, 0);
    R = Math.min(R, v);
    X = Math.min(X, S);
    const Fa = R - L, Ga = X - aa;
    console.assert(Fa <= Math.min(xa, v));
    console.assert(Ga <= Math.min(Ya, S));
    if (!(0 >= Fa || 0 >= Ga)) {
      R = Fa < xa || Ga < Ya;
      v = X = 1;
      var qa = Math.ceil(Fa * X), ra = Math.ceil(Ga * v);
      S = la.bc();
      qa > S && (X *= S / qa, qa = S);
      ra > S && (v *= S / ra, ra = S);
      w || (w = new m.DynamicRectanizer(S), w.reset(512, 512));
      S = w.addRect(qa, ra);
      0 > S && (c(), G.add(this), S = w.addRect(qa, ra), console.assert(0 <= S));
      var $b = S & 65535, ac = S >> 16;
      M.push({ha:this.T.slice(this.T.length - 6), image:q, Za:$b, $a:ac, cc:L, dc:aa, uc:qa, jb:ra, Aa:X, Ba:v, Ta:new Float32Array(K), Bb:new Float32Array(I), indices:new Uint16Array(F), hc:R, vb:q.Ja << 1 | (R ? 1 : 0),});
      T += K.length;
      W += F.length;
      var ya = this.B, od = a(H), pd = Math.max(0, E * this.G[this.G.length - 1]);
      this.I.push(function() {
        ya.save();
        ya.resetTransform();
        ya.globalCompositeOperation = od;
        ya.globalAlpha = pd;
        const bc = la.canvas();
        bc && ya.drawImage(bc, $b, ac, qa, ra, L, aa, Fa, Ga);
        ya.restore();
      });
    }
  }, _clipPath:function(q) {
    this.I.push(this.B.clip.bind(this.B, q.U, q.Va === y ? "evenodd" : "nonzero"));
  }, clear:function() {
    G.add(this);
    this.I.push(this.B.clearRect.bind(this.B, 0, 0, this.Ua.width, this.Ua.height));
  }, flush:function() {
  }, translate:function(q, v) {
    this.transform(1, 0, 0, 1, q, v);
  },});
  m.makeRenderer = function(q) {
    const v = new pa(q), H = v.B;
    return new Proxy(v, {get(E, K) {
      if ("function" === typeof E[K]) {
        return function(...I) {
          return E[K].apply(E, I);
        };
      }
      if ("function" === typeof H[K]) {
        if (-1 < ka.indexOf(K)) {
          throw Error("RiveException: Method call to '" + K + "()' is not allowed, as the renderer cannot immediately pass through the return                 values of any canvas 2d context methods.");
        }
        return function(...I) {
          v.I.push(H[K].bind(H, ...I));
        };
      }
      return E[K];
    }, set(E, K, I) {
      if (K in H) {
        return v.I.push(() => {
          H[K] = I;
        }), !0;
      }
    },});
  };
  m.decodeImage = function(q, v) {
    (new u({la:v})).decode(q);
  };
  m.renderFactory = {makeRenderPaint:function() {
    return new C();
  }, makeRenderPath:function() {
    return new r();
  }, makeRenderImage:function() {
    let q = Z;
    return new u({wa:() => {
      q.total++;
    }, la:() => {
      q.loaded++;
      if (q.loaded === q.total) {
        const v = q.ready;
        v && (v(), q.ready = null);
      }
    },});
  },};
  let J = m.load, Z = null;
  m.load = function(q, v, H = !0) {
    const E = new m.FallbackFileAssetLoader();
    void 0 !== v && E.addLoader(v);
    H && (v = new m.CDNFileAssetLoader(), E.addLoader(v));
    return new Promise(function(K) {
      let I = null;
      Z = {total:0, loaded:0, ready:function() {
        K(I);
      },};
      I = J(q, E);
      0 == Z.total && K(I);
    });
  };
  let qd = m.RendererWrapper.prototype.align;
  m.RendererWrapper.prototype.align = function(q, v, H, E, K = 1.0) {
    qd.call(this, q, v, H, E, K);
  };
  d = new ha();
  m.requestAnimationFrame = d.requestAnimationFrame.bind(d);
  m.cancelAnimationFrame = d.cancelAnimationFrame.bind(d);
  m.enableFPSCounter = d.Ob.bind(d);
  m.disableFPSCounter = d.Lb;
  d.ob = c;
  m.resolveAnimationFrame = c;
  m.cleanup = function() {
    w && w.delete();
  };
};
var na = Object.assign({}, m), oa = "./this.program", sa = "", ta, ua;
if (ea || fa) {
  fa ? sa = self.location.href : "undefined" != typeof document && document.currentScript && (sa = document.currentScript.src), _scriptName && (sa = _scriptName), sa.startsWith("blob:") ? sa = "" : sa = sa.substr(0, sa.replace(/[?#].*/, "").lastIndexOf("/") + 1), fa && (ua = a => {
    var b = new XMLHttpRequest();
    b.open("GET", a, !1);
    b.responseType = "arraybuffer";
    b.send(null);
    return new Uint8Array(b.response);
  }), ta = (a, b, c) => {
    if (va(a)) {
      var d = new XMLHttpRequest();
      d.open("GET", a, !0);
      d.responseType = "arraybuffer";
      d.onload = () => {
        200 == d.status || 0 == d.status && d.response ? b(d.response) : c();
      };
      d.onerror = c;
      d.send(null);
    } else {
      fetch(a, {credentials:"same-origin"}).then(e => e.ok ? e.arrayBuffer() : Promise.reject(Error(e.status + " : " + e.url))).then(b, c);
    }
  };
}
var wa = m.print || console.log.bind(console), za = m.printErr || console.error.bind(console);
Object.assign(m, na);
na = null;
m.thisProgram && (oa = m.thisProgram);
var Aa;
m.wasmBinary && (Aa = m.wasmBinary);
var Ba, Ca = !1, z, A, Da, Ea, B, D, Ha, Ia;
function Ja() {
  var a = Ba.buffer;
  m.HEAP8 = z = new Int8Array(a);
  m.HEAP16 = Da = new Int16Array(a);
  m.HEAPU8 = A = new Uint8Array(a);
  m.HEAPU16 = Ea = new Uint16Array(a);
  m.HEAP32 = B = new Int32Array(a);
  m.HEAPU32 = D = new Uint32Array(a);
  m.HEAPF32 = Ha = new Float32Array(a);
  m.HEAPF64 = Ia = new Float64Array(a);
}
var Ka = [], La = [], Ma = [];
function Na() {
  var a = m.preRun.shift();
  Ka.unshift(a);
}
var Oa = 0, Pa = null, Qa = null;
function Ra(a) {
  m.onAbort?.(a);
  a = "Aborted(" + a + ")";
  za(a);
  Ca = !0;
  a = new WebAssembly.RuntimeError(a + ". Build with -sASSERTIONS for more info.");
  ca(a);
  throw a;
}
var Sa = a => a.startsWith("data:application/octet-stream;base64,"), va = a => a.startsWith("file://"), Ta;
function Ua(a) {
  if (a == Ta && Aa) {
    return new Uint8Array(Aa);
  }
  if (ua) {
    return ua(a);
  }
  throw "both async and sync fetching of the wasm failed";
}
function Va(a) {
  return Aa ? Promise.resolve().then(() => Ua(a)) : new Promise((b, c) => {
    ta(a, d => b(new Uint8Array(d)), () => {
      try {
        b(Ua(a));
      } catch (d) {
        c(d);
      }
    });
  });
}
function Wa(a, b, c) {
  return Va(a).then(d => WebAssembly.instantiate(d, b)).then(c, d => {
    za(`failed to asynchronously prepare wasm: ${d}`);
    Ra(d);
  });
}
function Xa(a, b) {
  var c = Ta;
  return Aa || "function" != typeof WebAssembly.instantiateStreaming || Sa(c) || va(c) || "function" != typeof fetch ? Wa(c, a, b) : fetch(c, {credentials:"same-origin"}).then(d => WebAssembly.instantiateStreaming(d, a).then(b, function(e) {
    za(`wasm streaming compile failed: ${e}`);
    za("falling back to ArrayBuffer instantiation");
    return Wa(c, a, b);
  }));
}
var Za, $a, db = {491138:(a, b, c, d, e) => {
  if ("undefined" === typeof window || void 0 === (window.AudioContext || window.webkitAudioContext)) {
    return 0;
  }
  if ("undefined" === typeof window.h) {
    window.h = {za:0};
    window.h.J = {};
    window.h.J.xa = a;
    window.h.J.capture = b;
    window.h.J.Ka = c;
    window.h.ga = {};
    window.h.ga.stopped = d;
    window.h.ga.wb = e;
    let f = window.h;
    f.D = [];
    f.sc = function(g) {
      for (var k = 0; k < f.D.length; ++k) {
        if (null == f.D[k]) {
          return f.D[k] = g, k;
        }
      }
      f.D.push(g);
      return f.D.length - 1;
    };
    f.Ab = function(g) {
      for (f.D[g] = null; 0 < f.D.length;) {
        if (null == f.D[f.D.length - 1]) {
          f.D.pop();
        } else {
          break;
        }
      }
    };
    f.Pc = function(g) {
      for (var k = 0; k < f.D.length; ++k) {
        if (f.D[k] == g) {
          return f.Ab(k);
        }
      }
    };
    f.qa = function(g) {
      return f.D[g];
    };
    f.Sa = ["touchend", "click"];
    f.unlock = function() {
      for (var g = 0; g < f.D.length; ++g) {
        var k = f.D[g];
        null != k && null != k.L && k.state === f.ga.wb && k.L.resume().then(() => {
          ab(k.pb);
        }, p => {
          console.error("Failed to resume audiocontext", p);
        });
      }
      f.Sa.map(function(p) {
        document.removeEventListener(p, f.unlock, !0);
      });
    };
    f.Sa.map(function(g) {
      document.addEventListener(g, f.unlock, !0);
    });
  }
  window.h.za += 1;
  return 1;
}, 493316:() => {
  "undefined" !== typeof window.h && (window.h.Sa.map(function(a) {
    document.removeEventListener(a, window.h.unlock, !0);
  }), --window.h.za, 0 === window.h.za && delete window.h);
}, 493620:() => void 0 !== navigator.mediaDevices && void 0 !== navigator.mediaDevices.getUserMedia, 493724:() => {
  try {
    var a = new (window.AudioContext || window.webkitAudioContext)(), b = a.sampleRate;
    a.close();
    return b;
  } catch (c) {
    return 0;
  }
}, 493895:(a, b, c, d, e, f) => {
  if ("undefined" === typeof window.h) {
    return -1;
  }
  var g = {}, k = {};
  a == window.h.J.xa && 0 != c && (k.sampleRate = c);
  g.L = new (window.AudioContext || window.webkitAudioContext)(k);
  g.L.suspend();
  g.state = window.h.ga.stopped;
  c = 0;
  a != window.h.J.xa && (c = b);
  g.Z = g.L.createScriptProcessor(d, c, b);
  g.Z.onaudioprocess = function(p) {
    if (null == g.ra || 0 == g.ra.length) {
      g.ra = new Float32Array(Ha.buffer, e, d * b);
    }
    if (a == window.h.J.capture || a == window.h.J.Ka) {
      for (var n = 0; n < b; n += 1) {
        for (var t = p.inputBuffer.getChannelData(n), x = g.ra, y = 0; y < d; y += 1) {
          x[y * b + n] = t[y];
        }
      }
      bb(f, d, e);
    }
    if (a == window.h.J.xa || a == window.h.J.Ka) {
      for (cb(f, d, e), n = 0; n < p.outputBuffer.numberOfChannels; ++n) {
        for (t = p.outputBuffer.getChannelData(n), x = g.ra, y = 0; y < d; y += 1) {
          t[y] = x[y * b + n];
        }
      }
    } else {
      for (n = 0; n < p.outputBuffer.numberOfChannels; ++n) {
        p.outputBuffer.getChannelData(n).fill(0.0);
      }
    }
  };
  a != window.h.J.capture && a != window.h.J.Ka || navigator.mediaDevices.getUserMedia({audio:!0, video:!1}).then(function(p) {
    g.Ca = g.L.createMediaStreamSource(p);
    g.Ca.connect(g.Z);
    g.Z.connect(g.L.destination);
  }).catch(function(p) {
    console.log("Failed to get user media: " + p);
  });
  a == window.h.J.xa && g.Z.connect(g.L.destination);
  g.pb = f;
  return window.h.sc(g);
}, 496772:a => window.h.qa(a).L.sampleRate, 496845:a => {
  a = window.h.qa(a);
  void 0 !== a.Z && (a.Z.onaudioprocess = function() {
  }, a.Z.disconnect(), a.Z = void 0);
  void 0 !== a.Ca && (a.Ca.disconnect(), a.Ca = void 0);
  a.L.close();
  a.L = void 0;
  a.pb = void 0;
}, 497245:a => {
  window.h.Ab(a);
}, 497295:a => {
  a = window.h.qa(a);
  a.L.resume();
  a.state = window.h.ga.wb;
}, 497434:a => {
  a = window.h.qa(a);
  a.L.suspend();
  a.state = window.h.ga.stopped;
}}, eb = a => {
  for (; 0 < a.length;) {
    a.shift()(m);
  }
};
function fb() {
  var a = B[+gb >> 2];
  gb += 4;
  return a;
}
var hb = (a, b) => {
  for (var c = 0, d = a.length - 1; 0 <= d; d--) {
    var e = a[d];
    "." === e ? a.splice(d, 1) : ".." === e ? (a.splice(d, 1), c++) : c && (a.splice(d, 1), c--);
  }
  if (b) {
    for (; c; c--) {
      a.unshift("..");
    }
  }
  return a;
}, ib = a => {
  var b = "/" === a.charAt(0), c = "/" === a.substr(-1);
  (a = hb(a.split("/").filter(d => !!d), !b).join("/")) || b || (a = ".");
  a && c && (a += "/");
  return (b ? "/" : "") + a;
}, jb = a => {
  var b = /^(\/?|)([\s\S]*?)((?:\.{1,2}|[^\/]+?|)(\.[^.\/]*|))(?:[\/]*)$/.exec(a).slice(1);
  a = b[0];
  b = b[1];
  if (!a && !b) {
    return ".";
  }
  b &&= b.substr(0, b.length - 1);
  return a + b;
}, kb = a => {
  if ("/" === a) {
    return "/";
  }
  a = ib(a);
  a = a.replace(/\/$/, "");
  var b = a.lastIndexOf("/");
  return -1 === b ? a : a.substr(b + 1);
}, lb = () => {
  if ("object" == typeof crypto && "function" == typeof crypto.getRandomValues) {
    return a => crypto.getRandomValues(a);
  }
  Ra("initRandomDevice");
}, mb = a => (mb = lb())(a), nb = (...a) => {
  for (var b = "", c = !1, d = a.length - 1; -1 <= d && !c; d--) {
    c = 0 <= d ? a[d] : "/";
    if ("string" != typeof c) {
      throw new TypeError("Arguments to path.resolve must be strings");
    }
    if (!c) {
      return "";
    }
    b = c + "/" + b;
    c = "/" === c.charAt(0);
  }
  b = hb(b.split("/").filter(e => !!e), !c).join("/");
  return (c ? "/" : "") + b || ".";
}, ob = "undefined" != typeof TextDecoder ? new TextDecoder("utf8") : void 0, pb = (a, b, c) => {
  var d = b + c;
  for (c = b; a[c] && !(c >= d);) {
    ++c;
  }
  if (16 < c - b && a.buffer && ob) {
    return ob.decode(a.subarray(b, c));
  }
  for (d = ""; b < c;) {
    var e = a[b++];
    if (e & 128) {
      var f = a[b++] & 63;
      if (192 == (e & 224)) {
        d += String.fromCharCode((e & 31) << 6 | f);
      } else {
        var g = a[b++] & 63;
        e = 224 == (e & 240) ? (e & 15) << 12 | f << 6 | g : (e & 7) << 18 | f << 12 | g << 6 | a[b++] & 63;
        65536 > e ? d += String.fromCharCode(e) : (e -= 65536, d += String.fromCharCode(55296 | e >> 10, 56320 | e & 1023));
      }
    } else {
      d += String.fromCharCode(e);
    }
  }
  return d;
}, qb = [], rb = a => {
  for (var b = 0, c = 0; c < a.length; ++c) {
    var d = a.charCodeAt(c);
    127 >= d ? b++ : 2047 >= d ? b += 2 : 55296 <= d && 57343 >= d ? (b += 4, ++c) : b += 3;
  }
  return b;
}, sb = (a, b, c, d) => {
  if (!(0 < d)) {
    return 0;
  }
  var e = c;
  d = c + d - 1;
  for (var f = 0; f < a.length; ++f) {
    var g = a.charCodeAt(f);
    if (55296 <= g && 57343 >= g) {
      var k = a.charCodeAt(++f);
      g = 65536 + ((g & 1023) << 10) | k & 1023;
    }
    if (127 >= g) {
      if (c >= d) {
        break;
      }
      b[c++] = g;
    } else {
      if (2047 >= g) {
        if (c + 1 >= d) {
          break;
        }
        b[c++] = 192 | g >> 6;
      } else {
        if (65535 >= g) {
          if (c + 2 >= d) {
            break;
          }
          b[c++] = 224 | g >> 12;
        } else {
          if (c + 3 >= d) {
            break;
          }
          b[c++] = 240 | g >> 18;
          b[c++] = 128 | g >> 12 & 63;
        }
        b[c++] = 128 | g >> 6 & 63;
      }
      b[c++] = 128 | g & 63;
    }
  }
  b[c] = 0;
  return c - e;
};
function tb(a, b) {
  var c = Array(rb(a) + 1);
  a = sb(a, c, 0, c.length);
  b && (c.length = a);
  return c;
}
var ub = [];
function vb(a, b) {
  ub[a] = {input:[], H:[], W:b};
  wb(a, xb);
}
var xb = {open(a) {
  var b = ub[a.node.ya];
  if (!b) {
    throw new N(43);
  }
  a.s = b;
  a.seekable = !1;
}, close(a) {
  a.s.W.pa(a.s);
}, pa(a) {
  a.s.W.pa(a.s);
}, read(a, b, c, d) {
  if (!a.s || !a.s.W.ib) {
    throw new N(60);
  }
  for (var e = 0, f = 0; f < d; f++) {
    try {
      var g = a.s.W.ib(a.s);
    } catch (k) {
      throw new N(29);
    }
    if (void 0 === g && 0 === e) {
      throw new N(6);
    }
    if (null === g || void 0 === g) {
      break;
    }
    e++;
    b[c + f] = g;
  }
  e && (a.node.timestamp = Date.now());
  return e;
}, write(a, b, c, d) {
  if (!a.s || !a.s.W.Na) {
    throw new N(60);
  }
  try {
    for (var e = 0; e < d; e++) {
      a.s.W.Na(a.s, b[c + e]);
    }
  } catch (f) {
    throw new N(29);
  }
  d && (a.node.timestamp = Date.now());
  return e;
},}, yb = {ib() {
  a: {
    if (!qb.length) {
      var a = null;
      "undefined" != typeof window && "function" == typeof window.prompt && (a = window.prompt("Input: "), null !== a && (a += "\n"));
      if (!a) {
        a = null;
        break a;
      }
      qb = tb(a, !0);
    }
    a = qb.shift();
  }
  return a;
}, Na(a, b) {
  null === b || 10 === b ? (wa(pb(a.H, 0)), a.H = []) : 0 != b && a.H.push(b);
}, pa(a) {
  a.H && 0 < a.H.length && (wa(pb(a.H, 0)), a.H = []);
}, Yb() {
  return {Ac:25856, Cc:5, zc:191, Bc:35387, yc:[3, 28, 127, 21, 4, 0, 1, 0, 17, 19, 26, 0, 18, 15, 23, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]};
}, Zb() {
  return 0;
}, $b() {
  return [24, 80];
},}, zb = {Na(a, b) {
  null === b || 10 === b ? (za(pb(a.H, 0)), a.H = []) : 0 != b && a.H.push(b);
}, pa(a) {
  a.H && 0 < a.H.length && (za(pb(a.H, 0)), a.H = []);
},};
function Ab(a, b) {
  var c = a.l ? a.l.length : 0;
  c >= b || (b = Math.max(b, c * (1048576 > c ? 2.0 : 1.125) >>> 0), 0 != c && (b = Math.max(b, 256)), c = a.l, a.l = new Uint8Array(b), 0 < a.v && a.l.set(c.subarray(0, a.v), 0));
}
var O = {O:null, V() {
  return O.createNode(null, "/", 16895, 0);
}, createNode(a, b, c, d) {
  if (24576 === (c & 61440) || 4096 === (c & 61440)) {
    throw new N(63);
  }
  O.O || (O.O = {dir:{node:{Y:O.j.Y, R:O.j.R, ka:O.j.ka, ua:O.j.ua, tb:O.j.tb, zb:O.j.zb, ub:O.j.ub, sb:O.j.sb, Da:O.j.Da}, stream:{ba:O.m.ba}}, file:{node:{Y:O.j.Y, R:O.j.R}, stream:{ba:O.m.ba, read:O.m.read, write:O.m.write, Ya:O.m.Ya, lb:O.m.lb, nb:O.m.nb}}, link:{node:{Y:O.j.Y, R:O.j.R, ma:O.j.ma}, stream:{}}, ab:{node:{Y:O.j.Y, R:O.j.R}, stream:Bb}});
  c = Cb(a, b, c, d);
  16384 === (c.mode & 61440) ? (c.j = O.O.dir.node, c.m = O.O.dir.stream, c.l = {}) : 32768 === (c.mode & 61440) ? (c.j = O.O.file.node, c.m = O.O.file.stream, c.v = 0, c.l = null) : 40960 === (c.mode & 61440) ? (c.j = O.O.link.node, c.m = O.O.link.stream) : 8192 === (c.mode & 61440) && (c.j = O.O.ab.node, c.m = O.O.ab.stream);
  c.timestamp = Date.now();
  a && (a.l[b] = c, a.timestamp = c.timestamp);
  return c;
}, Gc(a) {
  return a.l ? a.l.subarray ? a.l.subarray(0, a.v) : new Uint8Array(a.l) : new Uint8Array(0);
}, j:{Y(a) {
  var b = {};
  b.Ec = 8192 === (a.mode & 61440) ? a.id : 1;
  b.Ic = a.id;
  b.mode = a.mode;
  b.Lc = 1;
  b.uid = 0;
  b.Hc = 0;
  b.ya = a.ya;
  16384 === (a.mode & 61440) ? b.size = 4096 : 32768 === (a.mode & 61440) ? b.size = a.v : 40960 === (a.mode & 61440) ? b.size = a.link.length : b.size = 0;
  b.wc = new Date(a.timestamp);
  b.Kc = new Date(a.timestamp);
  b.Dc = new Date(a.timestamp);
  b.Hb = 4096;
  b.xc = Math.ceil(b.size / b.Hb);
  return b;
}, R(a, b) {
  void 0 !== b.mode && (a.mode = b.mode);
  void 0 !== b.timestamp && (a.timestamp = b.timestamp);
  if (void 0 !== b.size && (b = b.size, a.v != b)) {
    if (0 == b) {
      a.l = null, a.v = 0;
    } else {
      var c = a.l;
      a.l = new Uint8Array(b);
      c && a.l.set(c.subarray(0, Math.min(b, a.v)));
      a.v = b;
    }
  }
}, ka() {
  throw Db[44];
}, ua(a, b, c, d) {
  return O.createNode(a, b, c, d);
}, tb(a, b, c) {
  if (16384 === (a.mode & 61440)) {
    try {
      var d = Eb(b, c);
    } catch (f) {
    }
    if (d) {
      for (var e in d.l) {
        throw new N(55);
      }
    }
  }
  delete a.parent.l[a.name];
  a.parent.timestamp = Date.now();
  a.name = c;
  b.l[c] = a;
  b.timestamp = a.parent.timestamp;
}, zb(a, b) {
  delete a.l[b];
  a.timestamp = Date.now();
}, ub(a, b) {
  var c = Eb(a, b), d;
  for (d in c.l) {
    throw new N(55);
  }
  delete a.l[b];
  a.timestamp = Date.now();
}, sb(a) {
  var b = [".", ".."], c;
  for (c of Object.keys(a.l)) {
    b.push(c);
  }
  return b;
}, Da(a, b, c) {
  a = O.createNode(a, b, 41471, 0);
  a.link = c;
  return a;
}, ma(a) {
  if (40960 !== (a.mode & 61440)) {
    throw new N(28);
  }
  return a.link;
},}, m:{read(a, b, c, d, e) {
  var f = a.node.l;
  if (e >= a.node.v) {
    return 0;
  }
  a = Math.min(a.node.v - e, d);
  if (8 < a && f.subarray) {
    b.set(f.subarray(e, e + a), c);
  } else {
    for (d = 0; d < a; d++) {
      b[c + d] = f[e + d];
    }
  }
  return a;
}, write(a, b, c, d, e, f) {
  b.buffer === z.buffer && (f = !1);
  if (!d) {
    return 0;
  }
  a = a.node;
  a.timestamp = Date.now();
  if (b.subarray && (!a.l || a.l.subarray)) {
    if (f) {
      return a.l = b.subarray(c, c + d), a.v = d;
    }
    if (0 === a.v && 0 === e) {
      return a.l = b.slice(c, c + d), a.v = d;
    }
    if (e + d <= a.v) {
      return a.l.set(b.subarray(c, c + d), e), d;
    }
  }
  Ab(a, e + d);
  if (a.l.subarray && b.subarray) {
    a.l.set(b.subarray(c, c + d), e);
  } else {
    for (f = 0; f < d; f++) {
      a.l[e + f] = b[c + f];
    }
  }
  a.v = Math.max(a.v, e + d);
  return d;
}, ba(a, b, c) {
  1 === c ? b += a.position : 2 === c && 32768 === (a.node.mode & 61440) && (b += a.node.v);
  if (0 > b) {
    throw new N(28);
  }
  return b;
}, Ya(a, b, c) {
  Ab(a.node, b + c);
  a.node.v = Math.max(a.node.v, b + c);
}, lb(a, b, c, d, e) {
  if (32768 !== (a.node.mode & 61440)) {
    throw new N(43);
  }
  a = a.node.l;
  if (e & 2 || a.buffer !== z.buffer) {
    if (0 < c || c + b < a.length) {
      a.subarray ? a = a.subarray(c, c + b) : a = Array.prototype.slice.call(a, c, c + b);
    }
    c = !0;
    Ra();
    b = void 0;
    if (!b) {
      throw new N(48);
    }
    z.set(a, b);
  } else {
    c = !1, b = a.byteOffset;
  }
  return {o:b, vc:c};
}, nb(a, b, c, d) {
  O.m.write(a, b, 0, d, c, !1);
  return 0;
},},}, Fb = (a, b) => {
  var c = 0;
  a && (c |= 365);
  b && (c |= 146);
  return c;
}, Gb = null, Hb = {}, Ib = [], Jb = 1, Kb = null, Lb = !0, N = class {
  constructor(a) {
    this.name = "ErrnoError";
    this.aa = a;
  }
}, Db = {}, Mb = class {
  constructor() {
    this.h = {};
    this.node = null;
  }
  get flags() {
    return this.h.flags;
  }
  set flags(a) {
    this.h.flags = a;
  }
  get position() {
    return this.h.position;
  }
  set position(a) {
    this.h.position = a;
  }
}, Nb = class {
  constructor(a, b, c, d) {
    a ||= this;
    this.parent = a;
    this.V = a.V;
    this.va = null;
    this.id = Jb++;
    this.name = b;
    this.mode = c;
    this.j = {};
    this.m = {};
    this.ya = d;
  }
  get read() {
    return 365 === (this.mode & 365);
  }
  set read(a) {
    a ? this.mode |= 365 : this.mode &= -366;
  }
  get write() {
    return 146 === (this.mode & 146);
  }
  set write(a) {
    a ? this.mode |= 146 : this.mode &= -147;
  }
};
function Ob(a, b = {}) {
  a = nb(a);
  if (!a) {
    return {path:"", node:null};
  }
  b = Object.assign({hb:!0, Pa:0}, b);
  if (8 < b.Pa) {
    throw new N(32);
  }
  a = a.split("/").filter(g => !!g);
  for (var c = Gb, d = "/", e = 0; e < a.length; e++) {
    var f = e === a.length - 1;
    if (f && b.parent) {
      break;
    }
    c = Eb(c, a[e]);
    d = ib(d + "/" + a[e]);
    c.va && (!f || f && b.hb) && (c = c.va.root);
    if (!f || b.gb) {
      for (f = 0; 40960 === (c.mode & 61440);) {
        if (c = Pb(d), d = nb(jb(d), c), c = Ob(d, {Pa:b.Pa + 1}).node, 40 < f++) {
          throw new N(32);
        }
      }
    }
  }
  return {path:d, node:c};
}
function Qb(a) {
  for (var b;;) {
    if (a === a.parent) {
      return a = a.V.mb, b ? "/" !== a[a.length - 1] ? `${a}/${b}` : a + b : a;
    }
    b = b ? `${a.name}/${b}` : a.name;
    a = a.parent;
  }
}
function Rb(a, b) {
  for (var c = 0, d = 0; d < b.length; d++) {
    c = (c << 5) - c + b.charCodeAt(d) | 0;
  }
  return (a + c >>> 0) % Kb.length;
}
function Eb(a, b) {
  var c = 16384 === (a.mode & 61440) ? (c = Sb(a, "x")) ? c : a.j.ka ? 0 : 2 : 54;
  if (c) {
    throw new N(c);
  }
  for (c = Kb[Rb(a.id, b)]; c; c = c.fc) {
    var d = c.name;
    if (c.parent.id === a.id && d === b) {
      return c;
    }
  }
  return a.j.ka(a, b);
}
function Cb(a, b, c, d) {
  a = new Nb(a, b, c, d);
  b = Rb(a.parent.id, a.name);
  a.fc = Kb[b];
  return Kb[b] = a;
}
function Tb(a) {
  var b = ["r", "w", "rw"][a & 3];
  a & 512 && (b += "w");
  return b;
}
function Sb(a, b) {
  if (Lb) {
    return 0;
  }
  if (!b.includes("r") || a.mode & 292) {
    if (b.includes("w") && !(a.mode & 146) || b.includes("x") && !(a.mode & 73)) {
      return 2;
    }
  } else {
    return 2;
  }
  return 0;
}
function Ub(a, b) {
  try {
    return Eb(a, b), 20;
  } catch (c) {
  }
  return Sb(a, "wx");
}
function Vb(a) {
  a = Ib[a];
  if (!a) {
    throw new N(8);
  }
  return a;
}
function Wb(a, b = -1) {
  a = Object.assign(new Mb(), a);
  if (-1 == b) {
    a: {
      for (b = 0; 4096 >= b; b++) {
        if (!Ib[b]) {
          break a;
        }
      }
      throw new N(33);
    }
  }
  a.X = b;
  return Ib[b] = a;
}
function Xb(a, b = -1) {
  a = Wb(a, b);
  a.m?.Fc?.(a);
  return a;
}
var Bb = {open(a) {
  a.m = Hb[a.node.ya].m;
  a.m.open?.(a);
}, ba() {
  throw new N(70);
},};
function wb(a, b) {
  Hb[a] = {m:b};
}
function Yb(a, b) {
  var c = "/" === b;
  if (c && Gb) {
    throw new N(10);
  }
  if (!c && b) {
    var d = Ob(b, {hb:!1});
    b = d.path;
    d = d.node;
    if (d.va) {
      throw new N(10);
    }
    if (16384 !== (d.mode & 61440)) {
      throw new N(54);
    }
  }
  b = {type:a, Nc:{}, mb:b, ec:[]};
  a = a.V(b);
  a.V = b;
  b.root = a;
  c ? Gb = a : d && (d.va = b, d.V && d.V.ec.push(b));
}
function Zb(a, b, c) {
  var d = Ob(a, {parent:!0}).node;
  a = kb(a);
  if (!a || "." === a || ".." === a) {
    throw new N(28);
  }
  var e = Ub(d, a);
  if (e) {
    throw new N(e);
  }
  if (!d.j.ua) {
    throw new N(63);
  }
  return d.j.ua(d, a, b, c);
}
function cc(a) {
  return Zb(a, 16895, 0);
}
function dc(a, b, c) {
  "undefined" == typeof c && (c = b, b = 438);
  Zb(a, b | 8192, c);
}
function ec(a, b) {
  if (!nb(a)) {
    throw new N(44);
  }
  var c = Ob(b, {parent:!0}).node;
  if (!c) {
    throw new N(44);
  }
  b = kb(b);
  var d = Ub(c, b);
  if (d) {
    throw new N(d);
  }
  if (!c.j.Da) {
    throw new N(63);
  }
  c.j.Da(c, b, a);
}
function Pb(a) {
  a = Ob(a).node;
  if (!a) {
    throw new N(44);
  }
  if (!a.j.ma) {
    throw new N(28);
  }
  return nb(Qb(a.parent), a.j.ma(a));
}
function fc(a, b, c) {
  if ("" === a) {
    throw new N(44);
  }
  if ("string" == typeof b) {
    var d = {r:0, "r+":2, w:577, "w+":578, a:1089, "a+":1090,}[b];
    if ("undefined" == typeof d) {
      throw Error(`Unknown file open mode: ${b}`);
    }
    b = d;
  }
  c = b & 64 ? ("undefined" == typeof c ? 438 : c) & 4095 | 32768 : 0;
  if ("object" == typeof a) {
    var e = a;
  } else {
    a = ib(a);
    try {
      e = Ob(a, {gb:!(b & 131072)}).node;
    } catch (f) {
    }
  }
  d = !1;
  if (b & 64) {
    if (e) {
      if (b & 128) {
        throw new N(20);
      }
    } else {
      e = Zb(a, c, 0), d = !0;
    }
  }
  if (!e) {
    throw new N(44);
  }
  8192 === (e.mode & 61440) && (b &= -513);
  if (b & 65536 && 16384 !== (e.mode & 61440)) {
    throw new N(54);
  }
  if (!d && (c = e ? 40960 === (e.mode & 61440) ? 32 : 16384 === (e.mode & 61440) && ("r" !== Tb(b) || b & 512) ? 31 : Sb(e, Tb(b)) : 44)) {
    throw new N(c);
  }
  if (b & 512 && !d) {
    c = e;
    c = "string" == typeof c ? Ob(c, {gb:!0}).node : c;
    if (!c.j.R) {
      throw new N(63);
    }
    if (16384 === (c.mode & 61440)) {
      throw new N(31);
    }
    if (32768 !== (c.mode & 61440)) {
      throw new N(28);
    }
    if (d = Sb(c, "w")) {
      throw new N(d);
    }
    c.j.R(c, {size:0, timestamp:Date.now()});
  }
  b &= -131713;
  e = Wb({node:e, path:Qb(e), flags:b, seekable:!0, position:0, m:e.m, tc:[], error:!1});
  e.m.open && e.m.open(e);
  !m.logReadFiles || b & 1 || (gc ||= {}, a in gc || (gc[a] = 1));
  return e;
}
function hc(a, b, c) {
  if (null === a.X) {
    throw new N(8);
  }
  if (!a.seekable || !a.m.ba) {
    throw new N(70);
  }
  if (0 != c && 1 != c && 2 != c) {
    throw new N(28);
  }
  a.position = a.m.ba(a, b, c);
  a.tc = [];
}
var ic;
function jc(a, b, c) {
  a = ib("/dev/" + a);
  var d = Fb(!!b, !!c);
  kc ||= 64;
  var e = kc++ << 8 | 0;
  wb(e, {open(f) {
    f.seekable = !1;
  }, close() {
    c?.buffer?.length && c(10);
  }, read(f, g, k, p) {
    for (var n = 0, t = 0; t < p; t++) {
      try {
        var x = b();
      } catch (y) {
        throw new N(29);
      }
      if (void 0 === x && 0 === n) {
        throw new N(6);
      }
      if (null === x || void 0 === x) {
        break;
      }
      n++;
      g[k + t] = x;
    }
    n && (f.node.timestamp = Date.now());
    return n;
  }, write(f, g, k, p) {
    for (var n = 0; n < p; n++) {
      try {
        c(g[k + n]);
      } catch (t) {
        throw new N(29);
      }
    }
    p && (f.node.timestamp = Date.now());
    return n;
  }});
  dc(a, d, e);
}
var kc, lc = {}, gc, gb = void 0, mc = (a, b) => Object.defineProperty(b, "name", {value:a}), nc = [], oc = [], P, pc = a => {
  if (!a) {
    throw new P("Cannot use deleted val. handle = " + a);
  }
  return oc[a];
}, qc = a => {
  switch(a) {
    case void 0:
      return 2;
    case null:
      return 4;
    case !0:
      return 6;
    case !1:
      return 8;
    default:
      const b = nc.pop() || oc.length;
      oc[b] = a;
      oc[b + 1] = 1;
      return b;
  }
}, rc = a => {
  var b = Error, c = mc(a, function(d) {
    this.name = a;
    this.message = d;
    d = Error(d).stack;
    void 0 !== d && (this.stack = this.toString() + "\n" + d.replace(/^Error(:[^\n]*)?\n/, ""));
  });
  c.prototype = Object.create(b.prototype);
  c.prototype.constructor = c;
  c.prototype.toString = function() {
    return void 0 === this.message ? this.name : `${this.name}: ${this.message}`;
  };
  return c;
}, sc, tc, Q = a => {
  for (var b = ""; A[a];) {
    b += tc[A[a++]];
  }
  return b;
}, uc = [], vc = () => {
  for (; uc.length;) {
    var a = uc.pop();
    a.g.fa = !1;
    a["delete"]();
  }
}, wc, xc = {}, yc = (a, b) => {
  if (void 0 === b) {
    throw new P("ptr should not be undefined");
  }
  for (; a.C;) {
    b = a.na(b), a = a.C;
  }
  return b;
}, zc = {}, Cc = a => {
  a = Ac(a);
  var b = Q(a);
  Bc(a);
  return b;
}, Dc = (a, b) => {
  var c = zc[a];
  if (void 0 === c) {
    throw a = `${b} has unknown type ${Cc(a)}`, new P(a);
  }
  return c;
}, Ec = () => {
}, Fc = !1, Gc = (a, b, c) => {
  if (b === c) {
    return a;
  }
  if (void 0 === c.C) {
    return null;
  }
  a = Gc(a, b, c.C);
  return null === a ? null : c.Mb(a);
}, Hc = {}, Ic = (a, b) => {
  b = yc(a, b);
  return xc[b];
}, Jc, Lc = (a, b) => {
  if (!b.u || !b.o) {
    throw new Jc("makeClassHandle requires ptr and ptrType");
  }
  if (!!b.K !== !!b.F) {
    throw new Jc("Both smartPtrType and smartPtr must be specified");
  }
  b.count = {value:1};
  return Kc(Object.create(a, {g:{value:b, writable:!0,},}));
}, Kc = a => {
  if ("undefined" === typeof FinalizationRegistry) {
    return Kc = b => b, a;
  }
  Fc = new FinalizationRegistry(b => {
    b = b.g;
    --b.count.value;
    0 === b.count.value && (b.F ? b.K.P(b.F) : b.u.i.P(b.o));
  });
  Kc = b => {
    var c = b.g;
    c.F && Fc.register(b, {g:c}, b);
    return b;
  };
  Ec = b => {
    Fc.unregister(b);
  };
  return Kc(a);
}, Mc = {}, Nc = a => {
  for (; a.length;) {
    var b = a.pop();
    a.pop()(b);
  }
};
function Oc(a) {
  return this.fromWireType(D[a >> 2]);
}
var Pc = {}, Qc = {}, U = (a, b, c) => {
  function d(k) {
    k = c(k);
    if (k.length !== a.length) {
      throw new Jc("Mismatched type converter count");
    }
    for (var p = 0; p < a.length; ++p) {
      Rc(a[p], k[p]);
    }
  }
  a.forEach(function(k) {
    Qc[k] = b;
  });
  var e = Array(b.length), f = [], g = 0;
  b.forEach((k, p) => {
    zc.hasOwnProperty(k) ? e[p] = zc[k] : (f.push(k), Pc.hasOwnProperty(k) || (Pc[k] = []), Pc[k].push(() => {
      e[p] = zc[k];
      ++g;
      g === f.length && d(e);
    }));
  });
  0 === f.length && d(e);
};
function Sc(a, b, c = {}) {
  var d = b.name;
  if (!a) {
    throw new P(`type "${d}" must have a positive integer typeid pointer`);
  }
  if (zc.hasOwnProperty(a)) {
    if (c.Wb) {
      return;
    }
    throw new P(`Cannot register type '${d}' twice`);
  }
  zc[a] = b;
  delete Qc[a];
  Pc.hasOwnProperty(a) && (b = Pc[a], delete Pc[a], b.forEach(e => e()));
}
function Rc(a, b, c = {}) {
  if (!("argPackAdvance" in b)) {
    throw new TypeError("registerType registeredInstance requires argPackAdvance");
  }
  return Sc(a, b, c);
}
var Tc = a => {
  throw new P(a.g.u.i.name + " instance already deleted");
};
function Uc() {
}
var Vc = (a, b, c) => {
  if (void 0 === a[b].A) {
    var d = a[b];
    a[b] = function(...e) {
      if (!a[b].A.hasOwnProperty(e.length)) {
        throw new P(`Function '${c}' called with an invalid number of arguments (${e.length}) - expects one of (${a[b].A})!`);
      }
      return a[b].A[e.length].apply(this, e);
    };
    a[b].A = [];
    a[b].A[d.ea] = d;
  }
}, Wc = (a, b, c) => {
  if (m.hasOwnProperty(a)) {
    if (void 0 === c || void 0 !== m[a].A && void 0 !== m[a].A[c]) {
      throw new P(`Cannot register public name '${a}' twice`);
    }
    Vc(m, a, a);
    if (m.hasOwnProperty(c)) {
      throw new P(`Cannot register multiple overloads of a function with the same number of arguments (${c})!`);
    }
    m[a].A[c] = b;
  } else {
    m[a] = b, void 0 !== c && (m[a].Mc = c);
  }
}, Xc = a => {
  if (void 0 === a) {
    return "_unknown";
  }
  a = a.replace(/[^a-zA-Z0-9_]/g, "$");
  var b = a.charCodeAt(0);
  return 48 <= b && 57 >= b ? `_${a}` : a;
};
function Yc(a, b, c, d, e, f, g, k) {
  this.name = a;
  this.constructor = b;
  this.N = c;
  this.P = d;
  this.C = e;
  this.Rb = f;
  this.na = g;
  this.Mb = k;
  this.qb = [];
}
var Zc = (a, b, c) => {
  for (; b !== c;) {
    if (!b.na) {
      throw new P(`Expected null or instance of ${c.name}, got an instance of ${b.name}`);
    }
    a = b.na(a);
    b = b.C;
  }
  return a;
};
function $c(a, b) {
  if (null === b) {
    if (this.Ma) {
      throw new P(`null is not a valid ${this.name}`);
    }
    return 0;
  }
  if (!b.g) {
    throw new P(`Cannot pass "${ad(b)}" as a ${this.name}`);
  }
  if (!b.g.o) {
    throw new P(`Cannot pass deleted object as a pointer of type ${this.name}`);
  }
  return Zc(b.g.o, b.g.u.i, this.i);
}
function bd(a, b) {
  if (null === b) {
    if (this.Ma) {
      throw new P(`null is not a valid ${this.name}`);
    }
    if (this.ta) {
      var c = this.Oa();
      null !== a && a.push(this.P, c);
      return c;
    }
    return 0;
  }
  if (!b || !b.g) {
    throw new P(`Cannot pass "${ad(b)}" as a ${this.name}`);
  }
  if (!b.g.o) {
    throw new P(`Cannot pass deleted object as a pointer of type ${this.name}`);
  }
  if (!this.sa && b.g.u.sa) {
    throw new P(`Cannot convert argument of type ${b.g.K ? b.g.K.name : b.g.u.name} to parameter type ${this.name}`);
  }
  c = Zc(b.g.o, b.g.u.i, this.i);
  if (this.ta) {
    if (void 0 === b.g.F) {
      throw new P("Passing raw pointer to smart pointer is illegal");
    }
    switch(this.nc) {
      case 0:
        if (b.g.K === this) {
          c = b.g.F;
        } else {
          throw new P(`Cannot convert argument of type ${b.g.K ? b.g.K.name : b.g.u.name} to parameter type ${this.name}`);
        }
        break;
      case 1:
        c = b.g.F;
        break;
      case 2:
        if (b.g.K === this) {
          c = b.g.F;
        } else {
          var d = b.clone();
          c = this.jc(c, qc(() => d["delete"]()));
          null !== a && a.push(this.P, c);
        }
        break;
      default:
        throw new P("Unsupporting sharing policy");
    }
  }
  return c;
}
function cd(a, b) {
  if (null === b) {
    if (this.Ma) {
      throw new P(`null is not a valid ${this.name}`);
    }
    return 0;
  }
  if (!b.g) {
    throw new P(`Cannot pass "${ad(b)}" as a ${this.name}`);
  }
  if (!b.g.o) {
    throw new P(`Cannot pass deleted object as a pointer of type ${this.name}`);
  }
  if (b.g.u.sa) {
    throw new P(`Cannot convert argument of type ${b.g.u.name} to parameter type ${this.name}`);
  }
  return Zc(b.g.o, b.g.u.i, this.i);
}
function dd(a, b, c, d, e, f, g, k, p, n, t) {
  this.name = a;
  this.i = b;
  this.Ma = c;
  this.sa = d;
  this.ta = e;
  this.ic = f;
  this.nc = g;
  this.rb = k;
  this.Oa = p;
  this.jc = n;
  this.P = t;
  e || void 0 !== b.C ? this.toWireType = bd : (this.toWireType = d ? $c : cd, this.M = null);
}
var ed = (a, b, c) => {
  if (!m.hasOwnProperty(a)) {
    throw new Jc("Replacing nonexistent public symbol");
  }
  void 0 !== m[a].A && void 0 !== c ? m[a].A[c] = b : (m[a] = b, m[a].ea = c);
}, fd = [], gd, hd = a => {
  var b = fd[a];
  b || (a >= fd.length && (fd.length = a + 1), fd[a] = b = gd.get(a));
  return b;
}, jd = (a, b, c = []) => {
  a.includes("j") ? (a = a.replace(/p/g, "i"), b = (0,m["dynCall_" + a])(b, ...c)) : b = hd(b)(...c);
  return b;
}, kd = (a, b) => (...c) => jd(a, b, c), V = (a, b) => {
  a = Q(a);
  var c = a.includes("j") ? kd(a, b) : hd(b);
  if ("function" != typeof c) {
    throw new P(`unknown function pointer with signature ${a}: ${b}`);
  }
  return c;
}, ld, md = (a, b) => {
  function c(f) {
    e[f] || zc[f] || (Qc[f] ? Qc[f].forEach(c) : (d.push(f), e[f] = !0));
  }
  var d = [], e = {};
  b.forEach(c);
  throw new ld(`${a}: ` + d.map(Cc).join([", "]));
};
function nd(a) {
  for (var b = 1; b < a.length; ++b) {
    if (null !== a[b] && void 0 === a[b].M) {
      return !0;
    }
  }
  return !1;
}
function rd(a, b, c, d, e) {
  var f = b.length;
  if (2 > f) {
    throw new P("argTypes array size mismatch! Must at least get return value and 'this' types!");
  }
  var g = null !== b[1] && null !== c, k = nd(b), p = "void" !== b[0].name, n = f - 2, t = Array(n), x = [], y = [];
  return mc(a, function(...l) {
    if (l.length !== n) {
      throw new P(`function ${a} called with ${l.length} arguments, expected ${n}`);
    }
    y.length = 0;
    x.length = g ? 2 : 1;
    x[0] = e;
    if (g) {
      var u = b[1].toWireType(y, this);
      x[1] = u;
    }
    for (var r = 0; r < n; ++r) {
      t[r] = b[r + 2].toWireType(y, l[r]), x.push(t[r]);
    }
    l = d(...x);
    if (k) {
      Nc(y);
    } else {
      for (r = g ? 1 : 2; r < b.length; r++) {
        var C = 1 === r ? u : t[r - 2];
        null !== b[r].M && b[r].M(C);
      }
    }
    u = p ? b[0].fromWireType(l) : void 0;
    return u;
  });
}
var sd = (a, b) => {
  for (var c = [], d = 0; d < a; d++) {
    c.push(D[b + 4 * d >> 2]);
  }
  return c;
}, td = a => {
  a = a.trim();
  const b = a.indexOf("(");
  return -1 !== b ? a.substr(0, b) : a;
}, ud = (a, b, c) => {
  if (!(a instanceof Object)) {
    throw new P(`${c} with invalid "this": ${a}`);
  }
  if (!(a instanceof b.i.constructor)) {
    throw new P(`${c} incompatible with "this" of type ${a.constructor.name}`);
  }
  if (!a.g.o) {
    throw new P(`cannot call emscripten binding method ${c} on deleted object`);
  }
  return Zc(a.g.o, a.g.u.i, b.i);
}, vd = a => {
  9 < a && 0 === --oc[a + 1] && (oc[a] = void 0, nc.push(a));
}, wd = {name:"emscripten::val", fromWireType:a => {
  var b = pc(a);
  vd(a);
  return b;
}, toWireType:(a, b) => qc(b), argPackAdvance:8, readValueFromPointer:Oc, M:null,}, xd = (a, b, c) => {
  switch(b) {
    case 1:
      return c ? function(d) {
        return this.fromWireType(z[d]);
      } : function(d) {
        return this.fromWireType(A[d]);
      };
    case 2:
      return c ? function(d) {
        return this.fromWireType(Da[d >> 1]);
      } : function(d) {
        return this.fromWireType(Ea[d >> 1]);
      };
    case 4:
      return c ? function(d) {
        return this.fromWireType(B[d >> 2]);
      } : function(d) {
        return this.fromWireType(D[d >> 2]);
      };
    default:
      throw new TypeError(`invalid integer width (${b}): ${a}`);
  }
}, ad = a => {
  if (null === a) {
    return "null";
  }
  var b = typeof a;
  return "object" === b || "array" === b || "function" === b ? a.toString() : "" + a;
}, yd = (a, b) => {
  switch(b) {
    case 4:
      return function(c) {
        return this.fromWireType(Ha[c >> 2]);
      };
    case 8:
      return function(c) {
        return this.fromWireType(Ia[c >> 3]);
      };
    default:
      throw new TypeError(`invalid float width (${b}): ${a}`);
  }
}, zd = (a, b, c) => {
  switch(b) {
    case 1:
      return c ? d => z[d] : d => A[d];
    case 2:
      return c ? d => Da[d >> 1] : d => Ea[d >> 1];
    case 4:
      return c ? d => B[d >> 2] : d => D[d >> 2];
    default:
      throw new TypeError(`invalid integer width (${b}): ${a}`);
  }
}, Ad = "undefined" != typeof TextDecoder ? new TextDecoder("utf-16le") : void 0, Bd = (a, b) => {
  var c = a >> 1;
  for (var d = c + b / 2; !(c >= d) && Ea[c];) {
    ++c;
  }
  c <<= 1;
  if (32 < c - a && Ad) {
    return Ad.decode(A.subarray(a, c));
  }
  c = "";
  for (d = 0; !(d >= b / 2); ++d) {
    var e = Da[a + 2 * d >> 1];
    if (0 == e) {
      break;
    }
    c += String.fromCharCode(e);
  }
  return c;
}, Cd = (a, b, c) => {
  c ??= 2147483647;
  if (2 > c) {
    return 0;
  }
  c -= 2;
  var d = b;
  c = c < 2 * a.length ? c / 2 : a.length;
  for (var e = 0; e < c; ++e) {
    Da[b >> 1] = a.charCodeAt(e), b += 2;
  }
  Da[b >> 1] = 0;
  return b - d;
}, Dd = a => 2 * a.length, Ed = (a, b) => {
  for (var c = 0, d = ""; !(c >= b / 4);) {
    var e = B[a + 4 * c >> 2];
    if (0 == e) {
      break;
    }
    ++c;
    65536 <= e ? (e -= 65536, d += String.fromCharCode(55296 | e >> 10, 56320 | e & 1023)) : d += String.fromCharCode(e);
  }
  return d;
}, Fd = (a, b, c) => {
  c ??= 2147483647;
  if (4 > c) {
    return 0;
  }
  var d = b;
  c = d + c - 4;
  for (var e = 0; e < a.length; ++e) {
    var f = a.charCodeAt(e);
    if (55296 <= f && 57343 >= f) {
      var g = a.charCodeAt(++e);
      f = 65536 + ((f & 1023) << 10) | g & 1023;
    }
    B[b >> 2] = f;
    b += 4;
    if (b + 4 > c) {
      break;
    }
  }
  B[b >> 2] = 0;
  return b - d;
}, Gd = a => {
  for (var b = 0, c = 0; c < a.length; ++c) {
    var d = a.charCodeAt(c);
    55296 <= d && 57343 >= d && ++c;
    b += 4;
  }
  return b;
}, Hd = (a, b, c) => {
  var d = [];
  a = a.toWireType(d, c);
  d.length && (D[b >> 2] = qc(d));
  return a;
}, Id = {}, Jd = a => {
  var b = Id[a];
  return void 0 === b ? Q(a) : b;
}, Kd = [], Ld = a => {
  var b = Kd.length;
  Kd.push(a);
  return b;
}, Md = (a, b) => {
  for (var c = Array(a), d = 0; d < a; ++d) {
    c[d] = Dc(D[b + 4 * d >> 2], "parameter " + d);
  }
  return c;
}, Nd = Reflect.construct, Od = a => 0 === a % 4 && (0 !== a % 100 || 0 === a % 400), Pd = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335], Qd = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334], Rd = [], Sd = {}, Ud = () => {
  if (!Td) {
    var a = {USER:"web_user", LOGNAME:"web_user", PATH:"/", PWD:"/", HOME:"/home/web_user", LANG:("object" == typeof navigator && navigator.languages && navigator.languages[0] || "C").replace("-", "_") + ".UTF-8", _:oa || "./this.program"}, b;
    for (b in Sd) {
      void 0 === Sd[b] ? delete a[b] : a[b] = Sd[b];
    }
    var c = [];
    for (b in a) {
      c.push(`${b}=${a[b]}`);
    }
    Td = c;
  }
  return Td;
}, Td, Vd = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], Wd = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], Xd = (a, b, c, d) => {
  function e(l, u, r) {
    for (l = "number" == typeof l ? l.toString() : l || ""; l.length < u;) {
      l = r[0] + l;
    }
    return l;
  }
  function f(l, u) {
    return e(l, u, "0");
  }
  function g(l, u) {
    function r(G) {
      return 0 > G ? -1 : 0 < G ? 1 : 0;
    }
    var C;
    0 === (C = r(l.getFullYear() - u.getFullYear())) && 0 === (C = r(l.getMonth() - u.getMonth())) && (C = r(l.getDate() - u.getDate()));
    return C;
  }
  function k(l) {
    switch(l.getDay()) {
      case 0:
        return new Date(l.getFullYear() - 1, 11, 29);
      case 1:
        return l;
      case 2:
        return new Date(l.getFullYear(), 0, 3);
      case 3:
        return new Date(l.getFullYear(), 0, 2);
      case 4:
        return new Date(l.getFullYear(), 0, 1);
      case 5:
        return new Date(l.getFullYear() - 1, 11, 31);
      case 6:
        return new Date(l.getFullYear() - 1, 11, 30);
    }
  }
  function p(l) {
    var u = l.ca;
    for (l = new Date((new Date(l.da + 1900, 0, 1)).getTime()); 0 < u;) {
      var r = l.getMonth(), C = (Od(l.getFullYear()) ? Vd : Wd)[r];
      if (u > C - l.getDate()) {
        u -= C - l.getDate() + 1, l.setDate(1), 11 > r ? l.setMonth(r + 1) : (l.setMonth(0), l.setFullYear(l.getFullYear() + 1));
      } else {
        l.setDate(l.getDate() + u);
        break;
      }
    }
    r = new Date(l.getFullYear() + 1, 0, 4);
    u = k(new Date(l.getFullYear(), 0, 4));
    r = k(r);
    return 0 >= g(u, l) ? 0 >= g(r, l) ? l.getFullYear() + 1 : l.getFullYear() : l.getFullYear() - 1;
  }
  var n = D[d + 40 >> 2];
  d = {qc:B[d >> 2], pc:B[d + 4 >> 2], Ea:B[d + 8 >> 2], Ra:B[d + 12 >> 2], Fa:B[d + 16 >> 2], da:B[d + 20 >> 2], S:B[d + 24 >> 2], ca:B[d + 28 >> 2], Oc:B[d + 32 >> 2], oc:B[d + 36 >> 2], rc:n ? n ? pb(A, n) : "" : ""};
  c = c ? pb(A, c) : "";
  n = {"%c":"%a %b %d %H:%M:%S %Y", "%D":"%m/%d/%y", "%F":"%Y-%m-%d", "%h":"%b", "%r":"%I:%M:%S %p", "%R":"%H:%M", "%T":"%H:%M:%S", "%x":"%m/%d/%y", "%X":"%H:%M:%S", "%Ec":"%c", "%EC":"%C", "%Ex":"%m/%d/%y", "%EX":"%H:%M:%S", "%Ey":"%y", "%EY":"%Y", "%Od":"%d", "%Oe":"%e", "%OH":"%H", "%OI":"%I", "%Om":"%m", "%OM":"%M", "%OS":"%S", "%Ou":"%u", "%OU":"%U", "%OV":"%V", "%Ow":"%w", "%OW":"%W", "%Oy":"%y",};
  for (var t in n) {
    c = c.replace(new RegExp(t, "g"), n[t]);
  }
  var x = "Sunday Monday Tuesday Wednesday Thursday Friday Saturday".split(" "), y = "January February March April May June July August September October November December".split(" ");
  n = {"%a":l => x[l.S].substring(0, 3), "%A":l => x[l.S], "%b":l => y[l.Fa].substring(0, 3), "%B":l => y[l.Fa], "%C":l => f((l.da + 1900) / 100 | 0, 2), "%d":l => f(l.Ra, 2), "%e":l => e(l.Ra, 2, " "), "%g":l => p(l).toString().substring(2), "%G":p, "%H":l => f(l.Ea, 2), "%I":l => {
    l = l.Ea;
    0 == l ? l = 12 : 12 < l && (l -= 12);
    return f(l, 2);
  }, "%j":l => {
    for (var u = 0, r = 0; r <= l.Fa - 1; u += (Od(l.da + 1900) ? Vd : Wd)[r++]) {
    }
    return f(l.Ra + u, 3);
  }, "%m":l => f(l.Fa + 1, 2), "%M":l => f(l.pc, 2), "%n":() => "\n", "%p":l => 0 <= l.Ea && 12 > l.Ea ? "AM" : "PM", "%S":l => f(l.qc, 2), "%t":() => "\t", "%u":l => l.S || 7, "%U":l => f(Math.floor((l.ca + 7 - l.S) / 7), 2), "%V":l => {
    var u = Math.floor((l.ca + 7 - (l.S + 6) % 7) / 7);
    2 >= (l.S + 371 - l.ca - 2) % 7 && u++;
    if (u) {
      53 == u && (r = (l.S + 371 - l.ca) % 7, 4 == r || 3 == r && Od(l.da) || (u = 1));
    } else {
      u = 52;
      var r = (l.S + 7 - l.ca - 1) % 7;
      (4 == r || 5 == r && Od(l.da % 400 - 1)) && u++;
    }
    return f(u, 2);
  }, "%w":l => l.S, "%W":l => f(Math.floor((l.ca + 7 - (l.S + 6) % 7) / 7), 2), "%y":l => (l.da + 1900).toString().substring(2), "%Y":l => l.da + 1900, "%z":l => {
    l = l.oc;
    var u = 0 <= l;
    l = Math.abs(l) / 60;
    return (u ? "+" : "-") + String("0000" + (l / 60 * 100 + l % 60)).slice(-4);
  }, "%Z":l => l.rc, "%%":() => "%"};
  c = c.replace(/%%/g, "\x00\x00");
  for (t in n) {
    c.includes(t) && (c = c.replace(new RegExp(t, "g"), n[t](d)));
  }
  c = c.replace(/\0\0/g, "%");
  t = tb(c, !1);
  if (t.length > b) {
    return 0;
  }
  z.set(t, a);
  return t.length - 1;
};
[44].forEach(a => {
  Db[a] = new N(a);
  Db[a].stack = "<generic error, no stack>";
});
Kb = Array(4096);
Yb(O, "/");
cc("/tmp");
cc("/home");
cc("/home/web_user");
(function() {
  cc("/dev");
  wb(259, {read:() => 0, write:(d, e, f, g) => g,});
  dc("/dev/null", 259);
  vb(1280, yb);
  vb(1536, zb);
  dc("/dev/tty", 1280);
  dc("/dev/tty1", 1536);
  var a = new Uint8Array(1024), b = 0, c = () => {
    0 === b && (b = mb(a).byteLength);
    return a[--b];
  };
  jc("random", c);
  jc("urandom", c);
  cc("/dev/shm");
  cc("/dev/shm/tmp");
})();
(function() {
  cc("/proc");
  var a = cc("/proc/self");
  cc("/proc/self/fd");
  Yb({V() {
    var b = Cb(a, "fd", 16895, 73);
    b.j = {ka(c, d) {
      var e = Vb(+d);
      c = {parent:null, V:{mb:"fake"}, j:{ma:() => e.path},};
      return c.parent = c;
    }};
    return b;
  }}, "/proc/self/fd");
})();
P = m.BindingError = class extends Error {
  constructor(a) {
    super(a);
    this.name = "BindingError";
  }
};
oc.push(0, 1, void 0, 1, null, 1, !0, 1, !1, 1,);
m.count_emval_handles = () => oc.length / 2 - 5 - nc.length;
sc = m.PureVirtualError = rc("PureVirtualError");
for (var Yd = Array(256), Zd = 0; 256 > Zd; ++Zd) {
  Yd[Zd] = String.fromCharCode(Zd);
}
tc = Yd;
m.getInheritedInstanceCount = () => Object.keys(xc).length;
m.getLiveInheritedInstances = () => {
  var a = [], b;
  for (b in xc) {
    xc.hasOwnProperty(b) && a.push(xc[b]);
  }
  return a;
};
m.flushPendingDeletes = vc;
m.setDelayFunction = a => {
  wc = a;
  uc.length && wc && wc(vc);
};
Jc = m.InternalError = class extends Error {
  constructor(a) {
    super(a);
    this.name = "InternalError";
  }
};
Object.assign(Uc.prototype, {isAliasOf:function(a) {
  if (!(this instanceof Uc && a instanceof Uc)) {
    return !1;
  }
  var b = this.g.u.i, c = this.g.o;
  a.g = a.g;
  var d = a.g.u.i;
  for (a = a.g.o; b.C;) {
    c = b.na(c), b = b.C;
  }
  for (; d.C;) {
    a = d.na(a), d = d.C;
  }
  return b === d && c === a;
}, clone:function() {
  this.g.o || Tc(this);
  if (this.g.ia) {
    return this.g.count.value += 1, this;
  }
  var a = Kc, b = Object, c = b.create, d = Object.getPrototypeOf(this), e = this.g;
  a = a(c.call(b, d, {g:{value:{count:e.count, fa:e.fa, ia:e.ia, o:e.o, u:e.u, F:e.F, K:e.K,},}}));
  a.g.count.value += 1;
  a.g.fa = !1;
  return a;
}, ["delete"]() {
  this.g.o || Tc(this);
  if (this.g.fa && !this.g.ia) {
    throw new P("Object already scheduled for deletion");
  }
  Ec(this);
  var a = this.g;
  --a.count.value;
  0 === a.count.value && (a.F ? a.K.P(a.F) : a.u.i.P(a.o));
  this.g.ia || (this.g.F = void 0, this.g.o = void 0);
}, isDeleted:function() {
  return !this.g.o;
}, deleteLater:function() {
  this.g.o || Tc(this);
  if (this.g.fa && !this.g.ia) {
    throw new P("Object already scheduled for deletion");
  }
  uc.push(this);
  1 === uc.length && wc && wc(vc);
  this.g.fa = !0;
  return this;
},});
Object.assign(dd.prototype, {Sb(a) {
  this.rb && (a = this.rb(a));
  return a;
}, bb(a) {
  this.P?.(a);
}, argPackAdvance:8, readValueFromPointer:Oc, fromWireType:function(a) {
  function b() {
    return this.ta ? Lc(this.i.N, {u:this.ic, o:c, K:this, F:a,}) : Lc(this.i.N, {u:this, o:a,});
  }
  var c = this.Sb(a);
  if (!c) {
    return this.bb(a), null;
  }
  var d = Ic(this.i, c);
  if (void 0 !== d) {
    if (0 === d.g.count.value) {
      return d.g.o = c, d.g.F = a, d.clone();
    }
    d = d.clone();
    this.bb(a);
    return d;
  }
  d = this.i.Rb(c);
  d = Hc[d];
  if (!d) {
    return b.call(this);
  }
  d = this.sa ? d.Ib : d.pointerType;
  var e = Gc(c, this.i, d.i);
  return null === e ? b.call(this) : this.ta ? Lc(d.i.N, {u:d, o:e, K:this, F:a,}) : Lc(d.i.N, {u:d, o:e,});
},});
ld = m.UnboundTypeError = rc("UnboundTypeError");
var be = {__syscall_fcntl64:function(a, b, c) {
  gb = c;
  try {
    var d = Vb(a);
    switch(b) {
      case 0:
        var e = fb();
        if (0 > e) {
          break;
        }
        for (; Ib[e];) {
          e++;
        }
        return Xb(d, e).X;
      case 1:
      case 2:
        return 0;
      case 3:
        return d.flags;
      case 4:
        return e = fb(), d.flags |= e, 0;
      case 12:
        return e = fb(), Da[e + 0 >> 1] = 2, 0;
      case 13:
      case 14:
        return 0;
    }
    return -28;
  } catch (f) {
    if ("undefined" == typeof lc || "ErrnoError" !== f.name) {
      throw f;
    }
    return -f.aa;
  }
}, __syscall_ioctl:function(a, b, c) {
  gb = c;
  try {
    var d = Vb(a);
    switch(b) {
      case 21509:
        return d.s ? 0 : -59;
      case 21505:
        if (!d.s) {
          return -59;
        }
        if (d.s.W.Yb) {
          a = [3, 28, 127, 21, 4, 0, 1, 0, 17, 19, 26, 0, 18, 15, 23, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,];
          var e = fb();
          B[e >> 2] = 25856;
          B[e + 4 >> 2] = 5;
          B[e + 8 >> 2] = 191;
          B[e + 12 >> 2] = 35387;
          for (var f = 0; 32 > f; f++) {
            z[e + f + 17] = a[f] || 0;
          }
        }
        return 0;
      case 21510:
      case 21511:
      case 21512:
        return d.s ? 0 : -59;
      case 21506:
      case 21507:
      case 21508:
        if (!d.s) {
          return -59;
        }
        if (d.s.W.Zb) {
          for (e = fb(), a = [], f = 0; 32 > f; f++) {
            a.push(z[e + f + 17]);
          }
        }
        return 0;
      case 21519:
        if (!d.s) {
          return -59;
        }
        e = fb();
        return B[e >> 2] = 0;
      case 21520:
        return d.s ? -28 : -59;
      case 21531:
        e = fb();
        if (!d.m.Xb) {
          throw new N(59);
        }
        return d.m.Xb(d, b, e);
      case 21523:
        if (!d.s) {
          return -59;
        }
        d.s.W.$b && (f = [24, 80], e = fb(), Da[e >> 1] = f[0], Da[e + 2 >> 1] = f[1]);
        return 0;
      case 21524:
        return d.s ? 0 : -59;
      case 21515:
        return d.s ? 0 : -59;
      default:
        return -28;
    }
  } catch (g) {
    if ("undefined" == typeof lc || "ErrnoError" !== g.name) {
      throw g;
    }
    return -g.aa;
  }
}, __syscall_openat:function(a, b, c, d) {
  gb = d;
  try {
    b = b ? pb(A, b) : "";
    var e = b;
    if ("/" === e.charAt(0)) {
      b = e;
    } else {
      var f = -100 === a ? "/" : Vb(a).path;
      if (0 == e.length) {
        throw new N(44);
      }
      b = ib(f + "/" + e);
    }
    var g = d ? fb() : 0;
    return fc(b, c, g).X;
  } catch (k) {
    if ("undefined" == typeof lc || "ErrnoError" !== k.name) {
      throw k;
    }
    return -k.aa;
  }
}, _abort_js:() => {
  Ra("");
}, _embind_create_inheriting_constructor:(a, b, c) => {
  a = Q(a);
  b = Dc(b, "wrapper");
  c = pc(c);
  var d = b.i, e = d.N, f = d.C.N, g = d.C.constructor;
  a = mc(a, function(...k) {
    d.C.qb.forEach(function(p) {
      if (this[p] === f[p]) {
        throw new sc(`Pure virtual function ${p} must be implemented in JavaScript`);
      }
    }.bind(this));
    Object.defineProperty(this, "__parent", {value:e});
    this.__construct(...k);
  });
  e.__construct = function(...k) {
    if (this === e) {
      throw new P("Pass correct 'this' to __construct");
    }
    k = g.implement(this, ...k);
    Ec(k);
    var p = k.g;
    k.notifyOnDestruction();
    p.ia = !0;
    Object.defineProperties(this, {g:{value:p}});
    Kc(this);
    k = p.o;
    k = yc(d, k);
    if (xc.hasOwnProperty(k)) {
      throw new P(`Tried to register registered instance: ${k}`);
    }
    xc[k] = this;
  };
  e.__destruct = function() {
    if (this === e) {
      throw new P("Pass correct 'this' to __destruct");
    }
    Ec(this);
    var k = this.g.o;
    k = yc(d, k);
    if (xc.hasOwnProperty(k)) {
      delete xc[k];
    } else {
      throw new P(`Tried to unregister unregistered instance: ${k}`);
    }
  };
  a.prototype = Object.create(e);
  Object.assign(a.prototype, c);
  return qc(a);
}, _embind_finalize_value_object:a => {
  var b = Mc[a];
  delete Mc[a];
  var c = b.Oa, d = b.P, e = b.fb, f = e.map(g => g.Vb).concat(e.map(g => g.lc));
  U([a], f, g => {
    var k = {};
    e.forEach((p, n) => {
      var t = g[n], x = p.Tb, y = p.Ub, l = g[n + e.length], u = p.kc, r = p.mc;
      k[p.Pb] = {read:C => t.fromWireType(x(y, C)), write:(C, G) => {
        var w = [];
        u(r, C, l.toWireType(w, G));
        Nc(w);
      }};
    });
    return [{name:b.name, fromWireType:p => {
      var n = {}, t;
      for (t in k) {
        n[t] = k[t].read(p);
      }
      d(p);
      return n;
    }, toWireType:(p, n) => {
      for (var t in k) {
        if (!(t in n)) {
          throw new TypeError(`Missing field: "${t}"`);
        }
      }
      var x = c();
      for (t in k) {
        k[t].write(x, n[t]);
      }
      null !== p && p.push(d, x);
      return x;
    }, argPackAdvance:8, readValueFromPointer:Oc, M:d,}];
  });
}, _embind_register_bigint:() => {
}, _embind_register_bool:(a, b, c, d) => {
  b = Q(b);
  Rc(a, {name:b, fromWireType:function(e) {
    return !!e;
  }, toWireType:function(e, f) {
    return f ? c : d;
  }, argPackAdvance:8, readValueFromPointer:function(e) {
    return this.fromWireType(A[e]);
  }, M:null,});
}, _embind_register_class:(a, b, c, d, e, f, g, k, p, n, t, x, y) => {
  t = Q(t);
  f = V(e, f);
  k &&= V(g, k);
  n &&= V(p, n);
  y = V(x, y);
  var l = Xc(t);
  Wc(l, function() {
    md(`Cannot construct ${t} due to unbound types`, [d]);
  });
  U([a, b, c], d ? [d] : [], u => {
    u = u[0];
    if (d) {
      var r = u.i;
      var C = r.N;
    } else {
      C = Uc.prototype;
    }
    u = mc(t, function(...T) {
      if (Object.getPrototypeOf(this) !== G) {
        throw new P("Use 'new' to construct " + t);
      }
      if (void 0 === w.$) {
        throw new P(t + " has no accessible constructor");
      }
      var W = w.$[T.length];
      if (void 0 === W) {
        throw new P(`Tried to invoke ctor of ${t} with invalid number of parameters (${T.length}) - expected (${Object.keys(w.$).toString()}) parameters instead!`);
      }
      return W.apply(this, T);
    });
    var G = Object.create(C, {constructor:{value:u},});
    u.prototype = G;
    var w = new Yc(t, u, G, y, r, f, k, n);
    if (w.C) {
      var M;
      (M = w.C).oa ?? (M.oa = []);
      w.C.oa.push(w);
    }
    r = new dd(t, w, !0, !1, !1);
    M = new dd(t + "*", w, !1, !1, !1);
    C = new dd(t + " const*", w, !1, !0, !1);
    Hc[a] = {pointerType:M, Ib:C};
    ed(l, u);
    return [r, M, C];
  });
}, _embind_register_class_class_function:(a, b, c, d, e, f, g) => {
  var k = sd(c, d);
  b = Q(b);
  b = td(b);
  f = V(e, f);
  U([], [a], p => {
    function n() {
      md(`Cannot call ${t} due to unbound types`, k);
    }
    p = p[0];
    var t = `${p.name}.${b}`;
    b.startsWith("@@") && (b = Symbol[b.substring(2)]);
    var x = p.i.constructor;
    void 0 === x[b] ? (n.ea = c - 1, x[b] = n) : (Vc(x, b, t), x[b].A[c - 1] = n);
    U([], k, y => {
      y = rd(t, [y[0], null].concat(y.slice(1)), null, f, g);
      void 0 === x[b].A ? (y.ea = c - 1, x[b] = y) : x[b].A[c - 1] = y;
      if (p.i.oa) {
        for (const l of p.i.oa) {
          l.constructor.hasOwnProperty(b) || (l.constructor[b] = y);
        }
      }
      return [];
    });
    return [];
  });
}, _embind_register_class_class_property:(a, b, c, d, e, f, g, k) => {
  b = Q(b);
  f = V(e, f);
  U([], [a], p => {
    p = p[0];
    var n = `${p.name}.${b}`, t = {get() {
      md(`Cannot access ${n} due to unbound types`, [c]);
    }, enumerable:!0, configurable:!0};
    t.set = k ? () => {
      md(`Cannot access ${n} due to unbound types`, [c]);
    } : () => {
      throw new P(`${n} is a read-only property`);
    };
    Object.defineProperty(p.i.constructor, b, t);
    U([], [c], x => {
      x = x[0];
      var y = {get() {
        return x.fromWireType(f(d));
      }, enumerable:!0};
      k && (k = V(g, k), y.set = l => {
        var u = [];
        k(d, x.toWireType(u, l));
        Nc(u);
      });
      Object.defineProperty(p.i.constructor, b, y);
      return [];
    });
    return [];
  });
}, _embind_register_class_constructor:(a, b, c, d, e, f) => {
  var g = sd(b, c);
  e = V(d, e);
  U([], [a], k => {
    k = k[0];
    var p = `constructor ${k.name}`;
    void 0 === k.i.$ && (k.i.$ = []);
    if (void 0 !== k.i.$[b - 1]) {
      throw new P(`Cannot register multiple constructors with identical number of parameters (${b - 1}) for class '${k.name}'! Overload resolution is currently only performed using the parameter count, not actual type info!`);
    }
    k.i.$[b - 1] = () => {
      md(`Cannot construct ${k.name} due to unbound types`, g);
    };
    U([], g, n => {
      n.splice(1, 0, null);
      k.i.$[b - 1] = rd(p, n, null, e, f);
      return [];
    });
    return [];
  });
}, _embind_register_class_function:(a, b, c, d, e, f, g, k) => {
  var p = sd(c, d);
  b = Q(b);
  b = td(b);
  f = V(e, f);
  U([], [a], n => {
    function t() {
      md(`Cannot call ${x} due to unbound types`, p);
    }
    n = n[0];
    var x = `${n.name}.${b}`;
    b.startsWith("@@") && (b = Symbol[b.substring(2)]);
    k && n.i.qb.push(b);
    var y = n.i.N, l = y[b];
    void 0 === l || void 0 === l.A && l.className !== n.name && l.ea === c - 2 ? (t.ea = c - 2, t.className = n.name, y[b] = t) : (Vc(y, b, x), y[b].A[c - 2] = t);
    U([], p, u => {
      u = rd(x, u, n, f, g);
      void 0 === y[b].A ? (u.ea = c - 2, y[b] = u) : y[b].A[c - 2] = u;
      return [];
    });
    return [];
  });
}, _embind_register_class_property:(a, b, c, d, e, f, g, k, p, n) => {
  b = Q(b);
  e = V(d, e);
  U([], [a], t => {
    t = t[0];
    var x = `${t.name}.${b}`, y = {get() {
      md(`Cannot access ${x} due to unbound types`, [c, g]);
    }, enumerable:!0, configurable:!0};
    y.set = p ? () => md(`Cannot access ${x} due to unbound types`, [c, g]) : () => {
      throw new P(x + " is a read-only property");
    };
    Object.defineProperty(t.i.N, b, y);
    U([], p ? [c, g] : [c], l => {
      var u = l[0], r = {get() {
        var G = ud(this, t, x + " getter");
        return u.fromWireType(e(f, G));
      }, enumerable:!0};
      if (p) {
        p = V(k, p);
        var C = l[1];
        r.set = function(G) {
          var w = ud(this, t, x + " setter"), M = [];
          p(n, w, C.toWireType(M, G));
          Nc(M);
        };
      }
      Object.defineProperty(t.i.N, b, r);
      return [];
    });
    return [];
  });
}, _embind_register_emval:a => Rc(a, wd), _embind_register_enum:(a, b, c, d) => {
  function e() {
  }
  b = Q(b);
  e.values = {};
  Rc(a, {name:b, constructor:e, fromWireType:function(f) {
    return this.constructor.values[f];
  }, toWireType:(f, g) => g.value, argPackAdvance:8, readValueFromPointer:xd(b, c, d), M:null,});
  Wc(b, e);
}, _embind_register_enum_value:(a, b, c) => {
  var d = Dc(a, "enum");
  b = Q(b);
  a = d.constructor;
  d = Object.create(d.constructor.prototype, {value:{value:c}, constructor:{value:mc(`${d.name}_${b}`, function() {
  })},});
  a.values[c] = d;
  a[b] = d;
}, _embind_register_float:(a, b, c) => {
  b = Q(b);
  Rc(a, {name:b, fromWireType:d => d, toWireType:(d, e) => e, argPackAdvance:8, readValueFromPointer:yd(b, c), M:null,});
}, _embind_register_function:(a, b, c, d, e, f) => {
  var g = sd(b, c);
  a = Q(a);
  a = td(a);
  e = V(d, e);
  Wc(a, function() {
    md(`Cannot call ${a} due to unbound types`, g);
  }, b - 1);
  U([], g, k => {
    ed(a, rd(a, [k[0], null].concat(k.slice(1)), null, e, f), b - 1);
    return [];
  });
}, _embind_register_integer:(a, b, c, d, e) => {
  b = Q(b);
  -1 === e && (e = 4294967295);
  e = k => k;
  if (0 === d) {
    var f = 32 - 8 * c;
    e = k => k << f >>> f;
  }
  var g = b.includes("unsigned") ? function(k, p) {
    return p >>> 0;
  } : function(k, p) {
    return p;
  };
  Rc(a, {name:b, fromWireType:e, toWireType:g, argPackAdvance:8, readValueFromPointer:zd(b, c, 0 !== d), M:null,});
}, _embind_register_memory_view:(a, b, c) => {
  function d(f) {
    return new e(z.buffer, D[f + 4 >> 2], D[f >> 2]);
  }
  var e = [Int8Array, Uint8Array, Int16Array, Uint16Array, Int32Array, Uint32Array, Float32Array, Float64Array,][b];
  c = Q(c);
  Rc(a, {name:c, fromWireType:d, argPackAdvance:8, readValueFromPointer:d,}, {Wb:!0,});
}, _embind_register_std_string:(a, b) => {
  b = Q(b);
  var c = "std::string" === b;
  Rc(a, {name:b, fromWireType:function(d) {
    var e = D[d >> 2], f = d + 4;
    if (c) {
      for (var g = f, k = 0; k <= e; ++k) {
        var p = f + k;
        if (k == e || 0 == A[p]) {
          g = g ? pb(A, g, p - g) : "";
          if (void 0 === n) {
            var n = g;
          } else {
            n += String.fromCharCode(0), n += g;
          }
          g = p + 1;
        }
      }
    } else {
      n = Array(e);
      for (k = 0; k < e; ++k) {
        n[k] = String.fromCharCode(A[f + k]);
      }
      n = n.join("");
    }
    Bc(d);
    return n;
  }, toWireType:function(d, e) {
    e instanceof ArrayBuffer && (e = new Uint8Array(e));
    var f = "string" == typeof e;
    if (!(f || e instanceof Uint8Array || e instanceof Uint8ClampedArray || e instanceof Int8Array)) {
      throw new P("Cannot pass non-string to std::string");
    }
    var g = c && f ? rb(e) : e.length;
    var k = $d(4 + g + 1), p = k + 4;
    D[k >> 2] = g;
    if (c && f) {
      sb(e, A, p, g + 1);
    } else {
      if (f) {
        for (f = 0; f < g; ++f) {
          var n = e.charCodeAt(f);
          if (255 < n) {
            throw Bc(p), new P("String has UTF-16 code units that do not fit in 8 bits");
          }
          A[p + f] = n;
        }
      } else {
        for (f = 0; f < g; ++f) {
          A[p + f] = e[f];
        }
      }
    }
    null !== d && d.push(Bc, k);
    return k;
  }, argPackAdvance:8, readValueFromPointer:Oc, M(d) {
    Bc(d);
  },});
}, _embind_register_std_wstring:(a, b, c) => {
  c = Q(c);
  if (2 === b) {
    var d = Bd;
    var e = Cd;
    var f = Dd;
    var g = k => Ea[k >> 1];
  } else {
    4 === b && (d = Ed, e = Fd, f = Gd, g = k => D[k >> 2]);
  }
  Rc(a, {name:c, fromWireType:k => {
    for (var p = D[k >> 2], n, t = k + 4, x = 0; x <= p; ++x) {
      var y = k + 4 + x * b;
      if (x == p || 0 == g(y)) {
        t = d(t, y - t), void 0 === n ? n = t : (n += String.fromCharCode(0), n += t), t = y + b;
      }
    }
    Bc(k);
    return n;
  }, toWireType:(k, p) => {
    if ("string" != typeof p) {
      throw new P(`Cannot pass non-string to C++ string type ${c}`);
    }
    var n = f(p), t = $d(4 + n + b);
    D[t >> 2] = n / b;
    e(p, t + 4, n + b);
    null !== k && k.push(Bc, t);
    return t;
  }, argPackAdvance:8, readValueFromPointer:Oc, M(k) {
    Bc(k);
  }});
}, _embind_register_value_object:(a, b, c, d, e, f) => {
  Mc[a] = {name:Q(b), Oa:V(c, d), P:V(e, f), fb:[],};
}, _embind_register_value_object_field:(a, b, c, d, e, f, g, k, p, n) => {
  Mc[a].fb.push({Pb:Q(b), Vb:c, Tb:V(d, e), Ub:f, lc:g, kc:V(k, p), mc:n,});
}, _embind_register_void:(a, b) => {
  b = Q(b);
  Rc(a, {Jc:!0, name:b, argPackAdvance:0, fromWireType:() => {
  }, toWireType:() => {
  },});
}, _emscripten_get_now_is_monotonic:() => 1, _emscripten_memcpy_js:(a, b, c) => A.copyWithin(a, b, b + c), _emscripten_throw_longjmp:() => {
  throw Infinity;
}, _emval_as:(a, b, c) => {
  a = pc(a);
  b = Dc(b, "emval::as");
  return Hd(b, c, a);
}, _emval_call_method:(a, b, c, d, e) => {
  a = Kd[a];
  b = pc(b);
  c = Jd(c);
  return a(b, b[c], d, e);
}, _emval_decref:vd, _emval_get_method_caller:(a, b, c) => {
  var d = Md(a, b), e = d.shift();
  a--;
  var f = Array(a);
  b = `methodCaller<(${d.map(g => g.name).join(", ")}) => ${e.name}>`;
  return Ld(mc(b, (g, k, p, n) => {
    for (var t = 0, x = 0; x < a; ++x) {
      f[x] = d[x].readValueFromPointer(n + t), t += d[x].argPackAdvance;
    }
    g = 1 === c ? Nd(k, f) : k.apply(g, f);
    return Hd(e, p, g);
  }));
}, _emval_get_module_property:a => {
  a = Jd(a);
  return qc(m[a]);
}, _emval_get_property:(a, b) => {
  a = pc(a);
  b = pc(b);
  return qc(a[b]);
}, _emval_incref:a => {
  9 < a && (oc[a + 1] += 1);
}, _emval_new_array:() => qc([]), _emval_new_cstring:a => qc(Jd(a)), _emval_new_object:() => qc({}), _emval_run_destructors:a => {
  var b = pc(a);
  Nc(b);
  vd(a);
}, _emval_set_property:(a, b, c) => {
  a = pc(a);
  b = pc(b);
  c = pc(c);
  a[b] = c;
}, _emval_take_value:(a, b) => {
  a = Dc(a, "_emval_take_value");
  a = a.readValueFromPointer(b);
  return qc(a);
}, _gmtime_js:function(a, b, c) {
  a = new Date(1000 * (b + 2097152 >>> 0 < 4194305 - !!a ? (a >>> 0) + 4294967296 * b : NaN));
  B[c >> 2] = a.getUTCSeconds();
  B[c + 4 >> 2] = a.getUTCMinutes();
  B[c + 8 >> 2] = a.getUTCHours();
  B[c + 12 >> 2] = a.getUTCDate();
  B[c + 16 >> 2] = a.getUTCMonth();
  B[c + 20 >> 2] = a.getUTCFullYear() - 1900;
  B[c + 24 >> 2] = a.getUTCDay();
  B[c + 28 >> 2] = (a.getTime() - Date.UTC(a.getUTCFullYear(), 0, 1, 0, 0, 0, 0)) / 864E5 | 0;
}, _localtime_js:function(a, b, c) {
  a = new Date(1000 * (b + 2097152 >>> 0 < 4194305 - !!a ? (a >>> 0) + 4294967296 * b : NaN));
  B[c >> 2] = a.getSeconds();
  B[c + 4 >> 2] = a.getMinutes();
  B[c + 8 >> 2] = a.getHours();
  B[c + 12 >> 2] = a.getDate();
  B[c + 16 >> 2] = a.getMonth();
  B[c + 20 >> 2] = a.getFullYear() - 1900;
  B[c + 24 >> 2] = a.getDay();
  B[c + 28 >> 2] = (Od(a.getFullYear()) ? Pd : Qd)[a.getMonth()] + a.getDate() - 1 | 0;
  B[c + 36 >> 2] = -(60 * a.getTimezoneOffset());
  b = (new Date(a.getFullYear(), 6, 1)).getTimezoneOffset();
  var d = (new Date(a.getFullYear(), 0, 1)).getTimezoneOffset();
  B[c + 32 >> 2] = (b != d && a.getTimezoneOffset() == Math.min(d, b)) | 0;
}, _tzset_js:(a, b, c, d) => {
  var e = (new Date()).getFullYear(), f = new Date(e, 0, 1), g = new Date(e, 6, 1);
  e = f.getTimezoneOffset();
  var k = g.getTimezoneOffset();
  D[a >> 2] = 60 * Math.max(e, k);
  B[b >> 2] = Number(e != k);
  a = p => p.toLocaleTimeString(void 0, {hour12:!1, timeZoneName:"short"}).split(" ")[1];
  f = a(f);
  g = a(g);
  k < e ? (sb(f, A, c, 17), sb(g, A, d, 17)) : (sb(f, A, d, 17), sb(g, A, c, 17));
}, emscripten_asm_const_int:(a, b, c) => {
  Rd.length = 0;
  for (var d; d = A[b++];) {
    var e = 105 != d;
    e &= 112 != d;
    c += e && c % 8 ? 4 : 0;
    Rd.push(112 == d ? D[c >> 2] : 105 == d ? B[c >> 2] : Ia[c >> 3]);
    c += e ? 8 : 4;
  }
  return db[a](...Rd);
}, emscripten_date_now:() => Date.now(), emscripten_get_now:() => performance.now(), emscripten_resize_heap:a => {
  var b = A.length;
  a >>>= 0;
  if (2147483648 < a) {
    return !1;
  }
  for (var c = 1; 4 >= c; c *= 2) {
    var d = b * (1 + 0.2 / c);
    d = Math.min(d, a + 100663296);
    var e = Math;
    d = Math.max(a, d);
    a: {
      e = (e.min.call(e, 2147483648, d + (65536 - d % 65536) % 65536) - Ba.buffer.byteLength + 65535) / 65536;
      try {
        Ba.grow(e);
        Ja();
        var f = 1;
        break a;
      } catch (g) {
      }
      f = void 0;
    }
    if (f) {
      return !0;
    }
  }
  return !1;
}, environ_get:(a, b) => {
  var c = 0;
  Ud().forEach((d, e) => {
    var f = b + c;
    e = D[a + 4 * e >> 2] = f;
    for (f = 0; f < d.length; ++f) {
      z[e++] = d.charCodeAt(f);
    }
    z[e] = 0;
    c += d.length + 1;
  });
  return 0;
}, environ_sizes_get:(a, b) => {
  var c = Ud();
  D[a >> 2] = c.length;
  var d = 0;
  c.forEach(e => d += e.length + 1);
  D[b >> 2] = d;
  return 0;
}, fd_close:function(a) {
  try {
    var b = Vb(a);
    if (null === b.X) {
      throw new N(8);
    }
    b.La && (b.La = null);
    try {
      b.m.close && b.m.close(b);
    } catch (c) {
      throw c;
    } finally {
      Ib[b.X] = null;
    }
    b.X = null;
    return 0;
  } catch (c) {
    if ("undefined" == typeof lc || "ErrnoError" !== c.name) {
      throw c;
    }
    return c.aa;
  }
}, fd_read:function(a, b, c, d) {
  try {
    a: {
      var e = Vb(a);
      a = b;
      for (var f, g = b = 0; g < c; g++) {
        var k = D[a >> 2], p = D[a + 4 >> 2];
        a += 8;
        var n = e, t = f, x = z;
        if (0 > p || 0 > t) {
          throw new N(28);
        }
        if (null === n.X) {
          throw new N(8);
        }
        if (1 === (n.flags & 2097155)) {
          throw new N(8);
        }
        if (16384 === (n.node.mode & 61440)) {
          throw new N(31);
        }
        if (!n.m.read) {
          throw new N(28);
        }
        var y = "undefined" != typeof t;
        if (!y) {
          t = n.position;
        } else if (!n.seekable) {
          throw new N(70);
        }
        var l = n.m.read(n, x, k, p, t);
        y || (n.position += l);
        var u = l;
        if (0 > u) {
          var r = -1;
          break a;
        }
        b += u;
        if (u < p) {
          break;
        }
        "undefined" != typeof f && (f += u);
      }
      r = b;
    }
    D[d >> 2] = r;
    return 0;
  } catch (C) {
    if ("undefined" == typeof lc || "ErrnoError" !== C.name) {
      throw C;
    }
    return C.aa;
  }
}, fd_seek:function(a, b, c, d, e) {
  b = c + 2097152 >>> 0 < 4194305 - !!b ? (b >>> 0) + 4294967296 * c : NaN;
  try {
    if (isNaN(b)) {
      return 61;
    }
    var f = Vb(a);
    hc(f, b, d);
    $a = [f.position >>> 0, (Za = f.position, 1.0 <= +Math.abs(Za) ? 0.0 < Za ? +Math.floor(Za / 4294967296.0) >>> 0 : ~~+Math.ceil((Za - +(~~Za >>> 0)) / 4294967296.0) >>> 0 : 0)];
    B[e >> 2] = $a[0];
    B[e + 4 >> 2] = $a[1];
    f.La && 0 === b && 0 === d && (f.La = null);
    return 0;
  } catch (g) {
    if ("undefined" == typeof lc || "ErrnoError" !== g.name) {
      throw g;
    }
    return g.aa;
  }
}, fd_write:function(a, b, c, d) {
  try {
    a: {
      var e = Vb(a);
      a = b;
      for (var f, g = b = 0; g < c; g++) {
        var k = D[a >> 2], p = D[a + 4 >> 2];
        a += 8;
        var n = e, t = k, x = p, y = f, l = z;
        if (0 > x || 0 > y) {
          throw new N(28);
        }
        if (null === n.X) {
          throw new N(8);
        }
        if (0 === (n.flags & 2097155)) {
          throw new N(8);
        }
        if (16384 === (n.node.mode & 61440)) {
          throw new N(31);
        }
        if (!n.m.write) {
          throw new N(28);
        }
        n.seekable && n.flags & 1024 && hc(n, 0, 2);
        var u = "undefined" != typeof y;
        if (!u) {
          y = n.position;
        } else if (!n.seekable) {
          throw new N(70);
        }
        var r = n.m.write(n, l, t, x, y, void 0);
        u || (n.position += r);
        var C = r;
        if (0 > C) {
          var G = -1;
          break a;
        }
        b += C;
        "undefined" != typeof f && (f += C);
      }
      G = b;
    }
    D[d >> 2] = G;
    return 0;
  } catch (w) {
    if ("undefined" == typeof lc || "ErrnoError" !== w.name) {
      throw w;
    }
    return w.aa;
  }
}, invoke_vii:ae, isWindowsBrowser:function() {
  return -1 < navigator.platform.indexOf("Win");
}, strftime:Xd, strftime_l:(a, b, c, d) => Xd(a, b, c, d)}, Y = function() {
  function a(c) {
    Y = c.exports;
    Ba = Y.memory;
    Ja();
    gd = Y.__indirect_function_table;
    La.unshift(Y.__wasm_call_ctors);
    Oa--;
    m.monitorRunDependencies?.(Oa);
    0 == Oa && (null !== Pa && (clearInterval(Pa), Pa = null), Qa && (c = Qa, Qa = null, c()));
    return Y;
  }
  var b = {env:be, wasi_snapshot_preview1:be,};
  Oa++;
  m.monitorRunDependencies?.(Oa);
  if (m.instantiateWasm) {
    try {
      return m.instantiateWasm(b, a);
    } catch (c) {
      za(`Module.instantiateWasm callback failed with error: ${c}`), ca(c);
    }
  }
  Ta ||= Sa("canvas_advanced.wasm") ? "canvas_advanced.wasm" : m.locateFile ? m.locateFile("canvas_advanced.wasm", sa) : sa + "canvas_advanced.wasm";
  Xa(b, function(c) {
    a(c.instance);
  }).catch(ca);
  return {};
}(), Bc = a => (Bc = Y.free)(a), $d = a => ($d = Y.malloc)(a), Ac = a => (Ac = Y.__getTypeName)(a), ab = m._ma_device__on_notification_unlocked = a => (ab = m._ma_device__on_notification_unlocked = Y.ma_device__on_notification_unlocked)(a);
m._ma_malloc_emscripten = (a, b) => (m._ma_malloc_emscripten = Y.ma_malloc_emscripten)(a, b);
m._ma_free_emscripten = (a, b) => (m._ma_free_emscripten = Y.ma_free_emscripten)(a, b);
var bb = m._ma_device_process_pcm_frames_capture__webaudio = (a, b, c) => (bb = m._ma_device_process_pcm_frames_capture__webaudio = Y.ma_device_process_pcm_frames_capture__webaudio)(a, b, c), cb = m._ma_device_process_pcm_frames_playback__webaudio = (a, b, c) => (cb = m._ma_device_process_pcm_frames_playback__webaudio = Y.ma_device_process_pcm_frames_playback__webaudio)(a, b, c), ce = (a, b) => (ce = Y.setThrew)(a, b), de = a => (de = Y._emscripten_stack_restore)(a), ee = () => (ee = Y.emscripten_stack_get_current)();
m.dynCall_iiji = (a, b, c, d, e) => (m.dynCall_iiji = Y.dynCall_iiji)(a, b, c, d, e);
m.dynCall_jiji = (a, b, c, d, e) => (m.dynCall_jiji = Y.dynCall_jiji)(a, b, c, d, e);
m.dynCall_iiiji = (a, b, c, d, e, f) => (m.dynCall_iiiji = Y.dynCall_iiiji)(a, b, c, d, e, f);
m.dynCall_iij = (a, b, c, d) => (m.dynCall_iij = Y.dynCall_iij)(a, b, c, d);
m.dynCall_jii = (a, b, c) => (m.dynCall_jii = Y.dynCall_jii)(a, b, c);
m.dynCall_viijii = (a, b, c, d, e, f, g) => (m.dynCall_viijii = Y.dynCall_viijii)(a, b, c, d, e, f, g);
m.dynCall_iiiiij = (a, b, c, d, e, f, g) => (m.dynCall_iiiiij = Y.dynCall_iiiiij)(a, b, c, d, e, f, g);
m.dynCall_iiiiijj = (a, b, c, d, e, f, g, k, p) => (m.dynCall_iiiiijj = Y.dynCall_iiiiijj)(a, b, c, d, e, f, g, k, p);
m.dynCall_iiiiiijj = (a, b, c, d, e, f, g, k, p, n) => (m.dynCall_iiiiiijj = Y.dynCall_iiiiiijj)(a, b, c, d, e, f, g, k, p, n);
function ae(a, b, c) {
  var d = ee();
  try {
    hd(a)(b, c);
  } catch (e) {
    de(d);
    if (e !== e + 0) {
      throw e;
    }
    ce(1, 0);
  }
}
var fe;
Qa = function ge() {
  fe || he();
  fe || (Qa = ge);
};
function he() {
  function a() {
    if (!fe && (fe = !0, m.calledRun = !0, !Ca)) {
      m.noFSInit || ic || (ic = !0, m.stdin = m.stdin, m.stdout = m.stdout, m.stderr = m.stderr, m.stdin ? jc("stdin", m.stdin) : ec("/dev/tty", "/dev/stdin"), m.stdout ? jc("stdout", null, m.stdout) : ec("/dev/tty", "/dev/stdout"), m.stderr ? jc("stderr", null, m.stderr) : ec("/dev/tty1", "/dev/stderr"), fc("/dev/stdin", 0), fc("/dev/stdout", 1), fc("/dev/stderr", 1));
      Lb = !1;
      eb(La);
      ba(m);
      if (m.onRuntimeInitialized) {
        m.onRuntimeInitialized();
      }
      if (m.postRun) {
        for ("function" == typeof m.postRun && (m.postRun = [m.postRun]); m.postRun.length;) {
          var b = m.postRun.shift();
          Ma.unshift(b);
        }
      }
      eb(Ma);
    }
  }
  if (!(0 < Oa)) {
    if (m.preRun) {
      for ("function" == typeof m.preRun && (m.preRun = [m.preRun]); m.preRun.length;) {
        Na();
      }
    }
    eb(Ka);
    0 < Oa || (m.setStatus ? (m.setStatus("Running..."), setTimeout(function() {
      setTimeout(function() {
        m.setStatus("");
      }, 1);
      a();
    }, 1)) : a());
  }
}
if (m.preInit) {
  for ("function" == typeof m.preInit && (m.preInit = [m.preInit]); 0 < m.preInit.length;) {
    m.preInit.pop()();
  }
}
he();
moduleRtn = da;



  return moduleRtn;
}
);
})();
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Rive);


/***/ }),
/* 2 */
/***/ ((module) => {

module.exports = /*#__PURE__*/JSON.parse('{"name":"@rive-app/canvas","version":"2.35.0","description":"Rive\'s canvas based web api.","main":"rive.js","homepage":"https://rive.app","repository":{"type":"git","url":"https://github.com/rive-app/rive-wasm/tree/master/js"},"keywords":["rive","animation"],"author":"Rive","contributors":["Luigi Rosso <luigi@rive.app> (https://rive.app)","Maxwell Talbot <max@rive.app> (https://rive.app)","Arthur Vivian <arthur@rive.app> (https://rive.app)","Umberto Sonnino <umberto@rive.app> (https://rive.app)","Matthew Sullivan <matt.j.sullivan@gmail.com> (mailto:matt.j.sullivan@gmail.com)"],"license":"MIT","files":["rive.js","rive.js.map","rive.wasm","rive_fallback.wasm","rive.d.ts","rive_advanced.mjs.d.ts"],"typings":"rive.d.ts","dependencies":{},"browser":{"fs":false,"path":false}}');

/***/ }),
/* 3 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   Animation: () => (/* reexport safe */ _Animation__WEBPACK_IMPORTED_MODULE_0__.Animation)
/* harmony export */ });
/* harmony import */ var _Animation__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(4);



/***/ }),
/* 4 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   Animation: () => (/* binding */ Animation)
/* harmony export */ });
/**
 * Represents an animation that can be played on an Artboard.
 * Wraps animations and instances from the runtime and keeps track of playback state.
 *
 * The `Animation` class manages the state and behavior of a single animation instance,
 * including its current time, loop count, and ability to scrub to a specific time.
 *
 * The class provides methods to advance the animation, apply its interpolated keyframe
 * values to the Artboard, and clean up the underlying animation instance when the
 * animation is no longer needed.
 */
var Animation = /** @class */ (function () {
    /**
     * Constructs a new animation
     * @constructor
     * @param {any} animation: runtime animation object
     * @param {any} instance: runtime animation instance object
     */
    function Animation(animation, artboard, runtime, playing) {
        this.animation = animation;
        this.artboard = artboard;
        this.playing = playing;
        this.loopCount = 0;
        /**
         * The time to which the animation should move to on the next render.
         * If not null, the animation will scrub to this time instead of advancing by the given time.
         */
        this.scrubTo = null;
        this.instance = new runtime.LinearAnimationInstance(animation, artboard);
    }
    Object.defineProperty(Animation.prototype, "name", {
        /**
         * Returns the animation's name
         */
        get: function () {
            return this.animation.name;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Animation.prototype, "time", {
        /**
         * Returns the animation's name
         */
        get: function () {
            return this.instance.time;
        },
        /**
         * Sets the animation's current time
         */
        set: function (value) {
            this.instance.time = value;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Animation.prototype, "loopValue", {
        /**
         * Returns the animation's loop type
         */
        get: function () {
            return this.animation.loopValue;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Animation.prototype, "needsScrub", {
        /**
         * Indicates whether the animation needs to be scrubbed.
         * @returns `true` if the animation needs to be scrubbed, `false` otherwise.
         */
        get: function () {
            return this.scrubTo !== null;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Advances the animation by the give time. If the animation needs scrubbing,
     * time is ignored and the stored scrub value is used.
     * @param time the time to advance the animation by if no scrubbing required
     */
    Animation.prototype.advance = function (time) {
        if (this.scrubTo === null) {
            this.instance.advance(time);
        }
        else {
            this.instance.time = 0;
            this.instance.advance(this.scrubTo);
            this.scrubTo = null;
        }
    };
    /**
     * Apply interpolated keyframe values to the artboard. This should be called after calling
     * .advance() on an animation instance so that new values are applied to properties.
     *
     * Note: This does not advance the artboard, which updates all objects on the artboard
     * @param mix - Mix value for the animation from 0 to 1
     */
    Animation.prototype.apply = function (mix) {
        this.instance.apply(mix);
    };
    /**
     * Deletes the backing Wasm animation instance; once this is called, this
     * animation is no more.
     */
    Animation.prototype.cleanup = function () {
        this.instance.delete();
    };
    return Animation;
}());



/***/ }),
/* 5 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   AudioAssetWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.AudioAssetWrapper),
/* harmony export */   AudioWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.AudioWrapper),
/* harmony export */   BLANK_URL: () => (/* reexport safe */ _sanitizeUrl__WEBPACK_IMPORTED_MODULE_1__.BLANK_URL),
/* harmony export */   CustomFileAssetLoaderWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.CustomFileAssetLoaderWrapper),
/* harmony export */   FileAssetWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.FileAssetWrapper),
/* harmony export */   FileFinalizer: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.FileFinalizer),
/* harmony export */   FontAssetWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.FontAssetWrapper),
/* harmony export */   FontWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.FontWrapper),
/* harmony export */   ImageAssetWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.ImageAssetWrapper),
/* harmony export */   ImageWrapper: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.ImageWrapper),
/* harmony export */   createFinalization: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.createFinalization),
/* harmony export */   finalizationRegistry: () => (/* reexport safe */ _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__.finalizationRegistry),
/* harmony export */   registerTouchInteractions: () => (/* reexport safe */ _registerTouchInteractions__WEBPACK_IMPORTED_MODULE_0__.registerTouchInteractions),
/* harmony export */   sanitizeUrl: () => (/* reexport safe */ _sanitizeUrl__WEBPACK_IMPORTED_MODULE_1__.sanitizeUrl)
/* harmony export */ });
/* harmony import */ var _registerTouchInteractions__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(6);
/* harmony import */ var _sanitizeUrl__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(7);
/* harmony import */ var _finalizationRegistry__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(8);





/***/ }),
/* 6 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   registerTouchInteractions: () => (/* binding */ registerTouchInteractions)
/* harmony export */ });
var _this = undefined;
/**
 * Returns the clientX and clientY properties from touch or mouse events. Also
 * calls preventDefault() on the event if it is a touchstart or touchmove to prevent
 * scrolling the page on mobile devices
 * @param event - Either a TouchEvent or a MouseEvent
 * @returns - Coordinates of the clientX and clientY properties from the touch/mouse event
 */
var getClientCoordinates = function (event, isTouchScrollEnabled, enableMultiTouch) {
    var _a, _b;
    var coordinates = [];
    if (["touchstart", "touchmove"].indexOf(event.type) > -1 &&
        ((_a = event.changedTouches) === null || _a === void 0 ? void 0 : _a.length)) {
        // This flag, if false, prevents touch events on the canvas default behavior
        // which may prevent scrolling if a drag motion on the canvas is performed
        if (!isTouchScrollEnabled) {
            event.preventDefault();
        }
        var cnt = 0;
        var totalTouches = enableMultiTouch
            ? event.changedTouches.length
            : 1;
        while (cnt < totalTouches) {
            var touch = event.changedTouches[cnt];
            coordinates.push({
                clientX: touch.clientX,
                clientY: touch.clientY,
                identifier: touch.identifier,
            });
            cnt++;
        }
    }
    else if (event.type === "touchend" &&
        ((_b = event.changedTouches) === null || _b === void 0 ? void 0 : _b.length)) {
        var cnt = 0;
        var totalTouches = enableMultiTouch
            ? event.changedTouches.length
            : 1;
        while (cnt < totalTouches) {
            var touch = event.changedTouches[cnt];
            coordinates.push({
                clientX: touch.clientX,
                clientY: touch.clientY,
                identifier: touch.identifier,
            });
            cnt++;
        }
    }
    else {
        coordinates.push({
            clientX: event.clientX,
            clientY: event.clientY,
            identifier: 0,
        });
    }
    return coordinates;
};
/**
 * Registers mouse move/up/down callback handlers on the canvas to send meaningful coordinates to
 * the state machine pointer move/up/down functions based on cursor interaction
 */
var registerTouchInteractions = function (_a) {
    var canvas = _a.canvas, artboard = _a.artboard, _b = _a.stateMachines, stateMachines = _b === void 0 ? [] : _b, renderer = _a.renderer, rive = _a.rive, fit = _a.fit, alignment = _a.alignment, _c = _a.isTouchScrollEnabled, isTouchScrollEnabled = _c === void 0 ? false : _c, _d = _a.dispatchPointerExit, dispatchPointerExit = _d === void 0 ? true : _d, _e = _a.enableMultiTouch, enableMultiTouch = _e === void 0 ? false : _e, _f = _a.layoutScaleFactor, layoutScaleFactor = _f === void 0 ? 1.0 : _f;
    if (!canvas ||
        !stateMachines.length ||
        !renderer ||
        !rive ||
        !artboard ||
        typeof window === "undefined") {
        return null;
    }
    /**
     * After a touchend event, some browsers may fire synthetic mouse events
     * (mouseover, mousedown, mousemove, mouseup) if the touch interaction did not cause
     * any default action (such as scrolling).
     *
     * This is done to simulate the behavior of a mouse for applications that do not support
     * touch events.
     *
     * We're keeping track of the previous event to not send the synthetic mouse events if the
     * touch event was a click (touchstart -> touchend).
     *
     * This is only needed when `isTouchScrollEnabled` is false
     * When true, `preventDefault()` is called which prevents this behaviour.
     **/
    var _prevEventType = null;
    var _syntheticEventsActive = false;
    var processEventCallback = function (event) {
        // Exit early out of all synthetic mouse events
        // https://stackoverflow.com/questions/9656990/how-to-prevent-simulated-mouse-events-in-mobile-browsers
        // https://stackoverflow.com/questions/25572070/javascript-touchend-versus-click-dilemma
        if (_syntheticEventsActive && event instanceof MouseEvent) {
            // Synthetic event finished
            if (event.type == "mouseup") {
                _syntheticEventsActive = false;
            }
            return;
        }
        // Test if it's a "touch click". This could cause the browser to send
        // synthetic mouse events.
        _syntheticEventsActive =
            isTouchScrollEnabled &&
                event.type === "touchend" &&
                _prevEventType === "touchstart";
        _prevEventType = event.type;
        var boundingRect = event.currentTarget.getBoundingClientRect();
        var coordinateSets = getClientCoordinates(event, isTouchScrollEnabled, enableMultiTouch);
        var forwardMatrix = rive.computeAlignment(fit, alignment, {
            minX: 0,
            minY: 0,
            maxX: boundingRect.width,
            maxY: boundingRect.height,
        }, artboard.bounds, layoutScaleFactor);
        var invertedMatrix = new rive.Mat2D();
        forwardMatrix.invert(invertedMatrix);
        coordinateSets.forEach(function (coordinateSet) {
            var clientX = coordinateSet.clientX;
            var clientY = coordinateSet.clientY;
            if (!clientX && !clientY) {
                return;
            }
            var canvasX = clientX - boundingRect.left;
            var canvasY = clientY - boundingRect.top;
            var canvasCoordinatesVector = new rive.Vec2D(canvasX, canvasY);
            var transformedVector = rive.mapXY(invertedMatrix, canvasCoordinatesVector);
            var transformedX = transformedVector.x();
            var transformedY = transformedVector.y();
            coordinateSet.transformedX = transformedX;
            coordinateSet.transformedY = transformedY;
            transformedVector.delete();
            canvasCoordinatesVector.delete();
        });
        invertedMatrix.delete();
        forwardMatrix.delete();
        switch (event.type) {
            /**
             * There's a 2px buffer for a hitRadius when translating the pointer coordinates
             * down to the state machine. In cases where the hitbox is about that much away
             * from the Artboard border, we don't have exact precision on determining pointer
             * exit. We're therefore adding to the translated coordinates on mouseout of a canvas
             * to ensure that we report the mouse has truly exited the hitarea.
             * https://github.com/rive-app/rive-cpp/blob/master/src/animation/state_machine_instance.cpp#L336
             *
             */
            case "mouseout":
                var _loop_1 = function (stateMachine) {
                    if (dispatchPointerExit) {
                        coordinateSets.forEach(function (coordinateSet) {
                            stateMachine.pointerExit(coordinateSet.transformedX, coordinateSet.transformedY, coordinateSet.identifier);
                        });
                    }
                    else {
                        coordinateSets.forEach(function (coordinateSet) {
                            stateMachine.pointerMove(coordinateSet.transformedX, coordinateSet.transformedY, coordinateSet.identifier);
                        });
                    }
                };
                for (var _i = 0, stateMachines_1 = stateMachines; _i < stateMachines_1.length; _i++) {
                    var stateMachine = stateMachines_1[_i];
                    _loop_1(stateMachine);
                }
                break;
            // Pointer moving/hovering on the canvas
            case "touchmove":
            case "mouseover":
            case "mousemove": {
                var _loop_2 = function (stateMachine) {
                    coordinateSets.forEach(function (coordinateSet) {
                        stateMachine.pointerMove(coordinateSet.transformedX, coordinateSet.transformedY, coordinateSet.identifier);
                    });
                };
                for (var _a = 0, stateMachines_2 = stateMachines; _a < stateMachines_2.length; _a++) {
                    var stateMachine = stateMachines_2[_a];
                    _loop_2(stateMachine);
                }
                break;
            }
            // Pointer click initiated but not released yet on the canvas
            case "touchstart":
            case "mousedown": {
                var _loop_3 = function (stateMachine) {
                    coordinateSets.forEach(function (coordinateSet) {
                        stateMachine.pointerDown(coordinateSet.transformedX, coordinateSet.transformedY, coordinateSet.identifier);
                    });
                };
                for (var _b = 0, stateMachines_3 = stateMachines; _b < stateMachines_3.length; _b++) {
                    var stateMachine = stateMachines_3[_b];
                    _loop_3(stateMachine);
                }
                break;
            }
            // Pointer click released on the canvas
            case "touchend": {
                var _loop_4 = function (stateMachine) {
                    coordinateSets.forEach(function (coordinateSet) {
                        stateMachine.pointerUp(coordinateSet.transformedX, coordinateSet.transformedY, coordinateSet.identifier);
                        stateMachine.pointerExit(coordinateSet.transformedX, coordinateSet.transformedY, coordinateSet.identifier);
                    });
                };
                for (var _c = 0, stateMachines_4 = stateMachines; _c < stateMachines_4.length; _c++) {
                    var stateMachine = stateMachines_4[_c];
                    _loop_4(stateMachine);
                }
                break;
            }
            case "mouseup": {
                var _loop_5 = function (stateMachine) {
                    coordinateSets.forEach(function (coordinateSet) {
                        stateMachine.pointerUp(coordinateSet.transformedX, coordinateSet.transformedY, coordinateSet.identifier);
                    });
                };
                for (var _d = 0, stateMachines_5 = stateMachines; _d < stateMachines_5.length; _d++) {
                    var stateMachine = stateMachines_5[_d];
                    _loop_5(stateMachine);
                }
                break;
            }
            default:
        }
    };
    var callback = processEventCallback.bind(_this);
    canvas.addEventListener("mouseover", callback);
    canvas.addEventListener("mouseout", callback);
    canvas.addEventListener("mousemove", callback);
    canvas.addEventListener("mousedown", callback);
    canvas.addEventListener("mouseup", callback);
    canvas.addEventListener("touchmove", callback, {
        passive: isTouchScrollEnabled,
    });
    canvas.addEventListener("touchstart", callback, {
        passive: isTouchScrollEnabled,
    });
    canvas.addEventListener("touchend", callback);
    return function () {
        canvas.removeEventListener("mouseover", callback);
        canvas.removeEventListener("mouseout", callback);
        canvas.removeEventListener("mousemove", callback);
        canvas.removeEventListener("mousedown", callback);
        canvas.removeEventListener("mouseup", callback);
        canvas.removeEventListener("touchmove", callback);
        canvas.removeEventListener("touchstart", callback);
        canvas.removeEventListener("touchend", callback);
    };
};


/***/ }),
/* 7 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   BLANK_URL: () => (/* binding */ BLANK_URL),
/* harmony export */   sanitizeUrl: () => (/* binding */ sanitizeUrl)
/* harmony export */ });
// Reference: https://github.com/braintree/sanitize-url/tree/main
var invalidProtocolRegex = /^([^\w]*)(javascript|data|vbscript)/im;
var htmlEntitiesRegex = /&#(\w+)(^\w|;)?/g;
var htmlCtrlEntityRegex = /&(newline|tab);/gi;
var ctrlCharactersRegex = /[\u0000-\u001F\u007F-\u009F\u2000-\u200D\uFEFF]/gim;
var urlSchemeRegex = /^.+(:|&colon;)/gim;
var relativeFirstCharacters = [".", "/"];
var BLANK_URL = "about:blank";
function isRelativeUrlWithoutProtocol(url) {
    return relativeFirstCharacters.indexOf(url[0]) > -1;
}
// adapted from https://stackoverflow.com/a/29824550/2601552
function decodeHtmlCharacters(str) {
    var removedNullByte = str.replace(ctrlCharactersRegex, "");
    return removedNullByte.replace(htmlEntitiesRegex, function (match, dec) {
        return String.fromCharCode(dec);
    });
}
function sanitizeUrl(url) {
    if (!url) {
        return BLANK_URL;
    }
    var sanitizedUrl = decodeHtmlCharacters(url)
        .replace(htmlCtrlEntityRegex, "")
        .replace(ctrlCharactersRegex, "")
        .trim();
    if (!sanitizedUrl) {
        return BLANK_URL;
    }
    if (isRelativeUrlWithoutProtocol(sanitizedUrl)) {
        return sanitizedUrl;
    }
    var urlSchemeParseResults = sanitizedUrl.match(urlSchemeRegex);
    if (!urlSchemeParseResults) {
        return sanitizedUrl;
    }
    var urlScheme = urlSchemeParseResults[0];
    if (invalidProtocolRegex.test(urlScheme)) {
        return BLANK_URL;
    }
    return sanitizedUrl;
}


/***/ }),
/* 8 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   AudioAssetWrapper: () => (/* binding */ AudioAssetWrapper),
/* harmony export */   AudioWrapper: () => (/* binding */ AudioWrapper),
/* harmony export */   CustomFileAssetLoaderWrapper: () => (/* binding */ CustomFileAssetLoaderWrapper),
/* harmony export */   FileAssetWrapper: () => (/* binding */ FileAssetWrapper),
/* harmony export */   FileFinalizer: () => (/* binding */ FileFinalizer),
/* harmony export */   FontAssetWrapper: () => (/* binding */ FontAssetWrapper),
/* harmony export */   FontWrapper: () => (/* binding */ FontWrapper),
/* harmony export */   ImageAssetWrapper: () => (/* binding */ ImageAssetWrapper),
/* harmony export */   ImageWrapper: () => (/* binding */ ImageWrapper),
/* harmony export */   createFinalization: () => (/* binding */ createFinalization),
/* harmony export */   finalizationRegistry: () => (/* binding */ finalizationRegistry)
/* harmony export */ });
var __extends = (undefined && undefined.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var FileFinalizer = /** @class */ (function () {
    function FileFinalizer(file) {
        this.selfUnref = false;
        this._file = file;
    }
    FileFinalizer.prototype.unref = function () {
        if (this._file) {
            this._file.unref();
        }
    };
    return FileFinalizer;
}());
var ObjectFinalizer = /** @class */ (function () {
    function ObjectFinalizer(finalizableObject) {
        this._finalizableObject = finalizableObject;
    }
    ObjectFinalizer.prototype.unref = function () {
        this._finalizableObject.unref();
    };
    return ObjectFinalizer;
}());
var AssetWrapper = /** @class */ (function () {
    function AssetWrapper() {
        this.selfUnref = false;
    }
    AssetWrapper.prototype.unref = function () { };
    return AssetWrapper;
}());
var ImageWrapper = /** @class */ (function (_super) {
    __extends(ImageWrapper, _super);
    function ImageWrapper(image) {
        var _this = _super.call(this) || this;
        _this._nativeImage = image;
        return _this;
    }
    Object.defineProperty(ImageWrapper.prototype, "nativeImage", {
        get: function () {
            return this._nativeImage;
        },
        enumerable: false,
        configurable: true
    });
    ImageWrapper.prototype.unref = function () {
        if (this.selfUnref) {
            this._nativeImage.unref();
        }
    };
    return ImageWrapper;
}(AssetWrapper));
var AudioWrapper = /** @class */ (function (_super) {
    __extends(AudioWrapper, _super);
    function AudioWrapper(audio) {
        var _this = _super.call(this) || this;
        _this._nativeAudio = audio;
        return _this;
    }
    Object.defineProperty(AudioWrapper.prototype, "nativeAudio", {
        get: function () {
            return this._nativeAudio;
        },
        enumerable: false,
        configurable: true
    });
    AudioWrapper.prototype.unref = function () {
        if (this.selfUnref) {
            this._nativeAudio.unref();
        }
    };
    return AudioWrapper;
}(AssetWrapper));
var FontWrapper = /** @class */ (function (_super) {
    __extends(FontWrapper, _super);
    function FontWrapper(font) {
        var _this = _super.call(this) || this;
        _this._nativeFont = font;
        return _this;
    }
    Object.defineProperty(FontWrapper.prototype, "nativeFont", {
        get: function () {
            return this._nativeFont;
        },
        enumerable: false,
        configurable: true
    });
    FontWrapper.prototype.unref = function () {
        if (this.selfUnref) {
            this._nativeFont.unref();
        }
    };
    return FontWrapper;
}(AssetWrapper));
var CustomFileAssetLoaderWrapper = /** @class */ (function () {
    function CustomFileAssetLoaderWrapper(runtime, loaderCallback) {
        this._assetLoaderCallback = loaderCallback;
        this.assetLoader = new runtime.CustomFileAssetLoader({
            loadContents: this.loadContents.bind(this),
        });
    }
    CustomFileAssetLoaderWrapper.prototype.loadContents = function (asset, bytes) {
        var assetWrapper;
        if (asset.isImage) {
            assetWrapper = new ImageAssetWrapper(asset);
        }
        else if (asset.isAudio) {
            assetWrapper = new AudioAssetWrapper(asset);
        }
        else if (asset.isFont) {
            assetWrapper = new FontAssetWrapper(asset);
        }
        return this._assetLoaderCallback(assetWrapper, bytes);
    };
    return CustomFileAssetLoaderWrapper;
}());
/**
 * Rive class representing a FileAsset with relevant metadata fields to describe
 * an asset associated wtih the Rive File
 */
var FileAssetWrapper = /** @class */ (function () {
    function FileAssetWrapper(nativeAsset) {
        this._nativeFileAsset = nativeAsset;
    }
    FileAssetWrapper.prototype.decode = function (bytes) {
        this._nativeFileAsset.decode(bytes);
    };
    Object.defineProperty(FileAssetWrapper.prototype, "name", {
        get: function () {
            return this._nativeFileAsset.name;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(FileAssetWrapper.prototype, "fileExtension", {
        get: function () {
            return this._nativeFileAsset.fileExtension;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(FileAssetWrapper.prototype, "uniqueFilename", {
        get: function () {
            return this._nativeFileAsset.uniqueFilename;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(FileAssetWrapper.prototype, "isAudio", {
        get: function () {
            return this._nativeFileAsset.isAudio;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(FileAssetWrapper.prototype, "isImage", {
        get: function () {
            return this._nativeFileAsset.isImage;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(FileAssetWrapper.prototype, "isFont", {
        get: function () {
            return this._nativeFileAsset.isFont;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(FileAssetWrapper.prototype, "cdnUuid", {
        get: function () {
            return this._nativeFileAsset.cdnUuid;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(FileAssetWrapper.prototype, "nativeFileAsset", {
        get: function () {
            return this._nativeFileAsset;
        },
        enumerable: false,
        configurable: true
    });
    return FileAssetWrapper;
}());
/**
 * Rive class extending the FileAsset that exposes a `setRenderImage()` API with a
 * decoded Image (via the `decodeImage()` API) to set a new Image on the Rive FileAsset
 */
var ImageAssetWrapper = /** @class */ (function (_super) {
    __extends(ImageAssetWrapper, _super);
    function ImageAssetWrapper() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    ImageAssetWrapper.prototype.setRenderImage = function (image) {
        this._nativeFileAsset.setRenderImage(image.nativeImage);
    };
    return ImageAssetWrapper;
}(FileAssetWrapper));
/**
 * Rive class extending the FileAsset that exposes a `setAudioSource()` API with a
 * decoded Audio (via the `decodeAudio()` API) to set a new Audio on the Rive FileAsset
 */
var AudioAssetWrapper = /** @class */ (function (_super) {
    __extends(AudioAssetWrapper, _super);
    function AudioAssetWrapper() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    AudioAssetWrapper.prototype.setAudioSource = function (audio) {
        this._nativeFileAsset.setAudioSource(audio.nativeAudio);
    };
    return AudioAssetWrapper;
}(FileAssetWrapper));
/**
 * Rive class extending the FileAsset that exposes a `setFont()` API with a
 * decoded Font (via the `decodeFont()` API) to set a new Font on the Rive FileAsset
 */
var FontAssetWrapper = /** @class */ (function (_super) {
    __extends(FontAssetWrapper, _super);
    function FontAssetWrapper() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    FontAssetWrapper.prototype.setFont = function (font) {
        this._nativeFileAsset.setFont(font.nativeFont);
    };
    return FontAssetWrapper;
}(FileAssetWrapper));
var FakeFinalizationRegistry = /** @class */ (function () {
    function FakeFinalizationRegistry(_) {
    }
    FakeFinalizationRegistry.prototype.register = function (object) {
        object.selfUnref = true;
    };
    FakeFinalizationRegistry.prototype.unregister = function (_) { };
    return FakeFinalizationRegistry;
}());
var MyFinalizationRegistry = typeof FinalizationRegistry !== "undefined"
    ? FinalizationRegistry
    : FakeFinalizationRegistry;
var finalizationRegistry = new MyFinalizationRegistry(function (ob) {
    ob === null || ob === void 0 ? void 0 : ob.unref();
});
var createFinalization = function (target, finalizable) {
    var finalizer = new ObjectFinalizer(finalizable);
    finalizationRegistry.register(target, finalizer);
};



/***/ })
/******/ 	]);
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
var __webpack_exports__ = {};
// This entry needs to be wrapped in an IIFE because it needs to be isolated against other modules in the chunk.
(() => {
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   Alignment: () => (/* binding */ Alignment),
/* harmony export */   DataEnum: () => (/* binding */ DataEnum),
/* harmony export */   DrawOptimizationOptions: () => (/* binding */ DrawOptimizationOptions),
/* harmony export */   EventType: () => (/* binding */ EventType),
/* harmony export */   Fit: () => (/* binding */ Fit),
/* harmony export */   Layout: () => (/* binding */ Layout),
/* harmony export */   LoopType: () => (/* binding */ LoopType),
/* harmony export */   Rive: () => (/* binding */ Rive),
/* harmony export */   RiveEventType: () => (/* binding */ RiveEventType),
/* harmony export */   RiveFile: () => (/* binding */ RiveFile),
/* harmony export */   RuntimeLoader: () => (/* binding */ RuntimeLoader),
/* harmony export */   StateMachineInput: () => (/* binding */ StateMachineInput),
/* harmony export */   StateMachineInputType: () => (/* binding */ StateMachineInputType),
/* harmony export */   Testing: () => (/* binding */ Testing),
/* harmony export */   ViewModel: () => (/* binding */ ViewModel),
/* harmony export */   ViewModelInstance: () => (/* binding */ ViewModelInstance),
/* harmony export */   ViewModelInstanceArtboard: () => (/* binding */ ViewModelInstanceArtboard),
/* harmony export */   ViewModelInstanceAssetImage: () => (/* binding */ ViewModelInstanceAssetImage),
/* harmony export */   ViewModelInstanceBoolean: () => (/* binding */ ViewModelInstanceBoolean),
/* harmony export */   ViewModelInstanceColor: () => (/* binding */ ViewModelInstanceColor),
/* harmony export */   ViewModelInstanceEnum: () => (/* binding */ ViewModelInstanceEnum),
/* harmony export */   ViewModelInstanceList: () => (/* binding */ ViewModelInstanceList),
/* harmony export */   ViewModelInstanceNumber: () => (/* binding */ ViewModelInstanceNumber),
/* harmony export */   ViewModelInstanceString: () => (/* binding */ ViewModelInstanceString),
/* harmony export */   ViewModelInstanceTrigger: () => (/* binding */ ViewModelInstanceTrigger),
/* harmony export */   ViewModelInstanceValue: () => (/* binding */ ViewModelInstanceValue),
/* harmony export */   decodeAudio: () => (/* binding */ decodeAudio),
/* harmony export */   decodeFont: () => (/* binding */ decodeFont),
/* harmony export */   decodeImage: () => (/* binding */ decodeImage)
/* harmony export */ });
/* harmony import */ var _rive_advanced_mjs__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(1);
/* harmony import */ var package_json__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(2);
/* harmony import */ var _animation__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(3);
/* harmony import */ var _utils__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(5);
var __extends = (undefined && undefined.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var __assign = (undefined && undefined.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __awaiter = (undefined && undefined.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (undefined && undefined.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __spreadArray = (undefined && undefined.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};




var RiveError = /** @class */ (function (_super) {
    __extends(RiveError, _super);
    function RiveError() {
        var _this = _super !== null && _super.apply(this, arguments) || this;
        _this.isHandledError = true;
        return _this;
    }
    return RiveError;
}(Error));
// #regions helpers
var resolveErrorMessage = function (error) {
    return error && error.isHandledError
        ? error.message
        : "Problem loading file; may be corrupt!";
};
// #region layout
// Fit options for the canvas
var Fit;
(function (Fit) {
    Fit["Cover"] = "cover";
    Fit["Contain"] = "contain";
    Fit["Fill"] = "fill";
    Fit["FitWidth"] = "fitWidth";
    Fit["FitHeight"] = "fitHeight";
    Fit["None"] = "none";
    Fit["ScaleDown"] = "scaleDown";
    Fit["Layout"] = "layout";
})(Fit || (Fit = {}));
// Alignment options for the canvas
var Alignment;
(function (Alignment) {
    Alignment["Center"] = "center";
    Alignment["TopLeft"] = "topLeft";
    Alignment["TopCenter"] = "topCenter";
    Alignment["TopRight"] = "topRight";
    Alignment["CenterLeft"] = "centerLeft";
    Alignment["CenterRight"] = "centerRight";
    Alignment["BottomLeft"] = "bottomLeft";
    Alignment["BottomCenter"] = "bottomCenter";
    Alignment["BottomRight"] = "bottomRight";
})(Alignment || (Alignment = {}));
// Drawing optimization options
var DrawOptimizationOptions;
(function (DrawOptimizationOptions) {
    DrawOptimizationOptions["AlwaysDraw"] = "alwaysDraw";
    DrawOptimizationOptions["DrawOnChanged"] = "drawOnChanged";
})(DrawOptimizationOptions || (DrawOptimizationOptions = {}));
// Alignment options for Rive animations in a HTML canvas
var Layout = /** @class */ (function () {
    function Layout(params) {
        var _a, _b, _c, _d, _e, _f, _g;
        this.fit = (_a = params === null || params === void 0 ? void 0 : params.fit) !== null && _a !== void 0 ? _a : Fit.Contain;
        this.alignment = (_b = params === null || params === void 0 ? void 0 : params.alignment) !== null && _b !== void 0 ? _b : Alignment.Center;
        this.layoutScaleFactor = (_c = params === null || params === void 0 ? void 0 : params.layoutScaleFactor) !== null && _c !== void 0 ? _c : 1;
        this.minX = (_d = params === null || params === void 0 ? void 0 : params.minX) !== null && _d !== void 0 ? _d : 0;
        this.minY = (_e = params === null || params === void 0 ? void 0 : params.minY) !== null && _e !== void 0 ? _e : 0;
        this.maxX = (_f = params === null || params === void 0 ? void 0 : params.maxX) !== null && _f !== void 0 ? _f : 0;
        this.maxY = (_g = params === null || params === void 0 ? void 0 : params.maxY) !== null && _g !== void 0 ? _g : 0;
    }
    // Alternative constructor to build a Layout from an interface/object
    Layout.new = function (_a) {
        var fit = _a.fit, alignment = _a.alignment, minX = _a.minX, minY = _a.minY, maxX = _a.maxX, maxY = _a.maxY;
        console.warn("This function is deprecated: please use `new Layout({})` instead");
        return new Layout({ fit: fit, alignment: alignment, minX: minX, minY: minY, maxX: maxX, maxY: maxY });
    };
    /**
     * Makes a copy of the layout, replacing any specified parameters
     */
    Layout.prototype.copyWith = function (_a) {
        var fit = _a.fit, alignment = _a.alignment, layoutScaleFactor = _a.layoutScaleFactor, minX = _a.minX, minY = _a.minY, maxX = _a.maxX, maxY = _a.maxY;
        return new Layout({
            fit: fit !== null && fit !== void 0 ? fit : this.fit,
            alignment: alignment !== null && alignment !== void 0 ? alignment : this.alignment,
            layoutScaleFactor: layoutScaleFactor !== null && layoutScaleFactor !== void 0 ? layoutScaleFactor : this.layoutScaleFactor,
            minX: minX !== null && minX !== void 0 ? minX : this.minX,
            minY: minY !== null && minY !== void 0 ? minY : this.minY,
            maxX: maxX !== null && maxX !== void 0 ? maxX : this.maxX,
            maxY: maxY !== null && maxY !== void 0 ? maxY : this.maxY,
        });
    };
    // Returns fit for the Wasm runtime format
    Layout.prototype.runtimeFit = function (rive) {
        if (this.cachedRuntimeFit)
            return this.cachedRuntimeFit;
        var fit;
        if (this.fit === Fit.Cover)
            fit = rive.Fit.cover;
        else if (this.fit === Fit.Contain)
            fit = rive.Fit.contain;
        else if (this.fit === Fit.Fill)
            fit = rive.Fit.fill;
        else if (this.fit === Fit.FitWidth)
            fit = rive.Fit.fitWidth;
        else if (this.fit === Fit.FitHeight)
            fit = rive.Fit.fitHeight;
        else if (this.fit === Fit.ScaleDown)
            fit = rive.Fit.scaleDown;
        else if (this.fit === Fit.Layout)
            fit = rive.Fit.layout;
        else
            fit = rive.Fit.none;
        this.cachedRuntimeFit = fit;
        return fit;
    };
    // Returns alignment for the Wasm runtime format
    Layout.prototype.runtimeAlignment = function (rive) {
        if (this.cachedRuntimeAlignment)
            return this.cachedRuntimeAlignment;
        var alignment;
        if (this.alignment === Alignment.TopLeft)
            alignment = rive.Alignment.topLeft;
        else if (this.alignment === Alignment.TopCenter)
            alignment = rive.Alignment.topCenter;
        else if (this.alignment === Alignment.TopRight)
            alignment = rive.Alignment.topRight;
        else if (this.alignment === Alignment.CenterLeft)
            alignment = rive.Alignment.centerLeft;
        else if (this.alignment === Alignment.CenterRight)
            alignment = rive.Alignment.centerRight;
        else if (this.alignment === Alignment.BottomLeft)
            alignment = rive.Alignment.bottomLeft;
        else if (this.alignment === Alignment.BottomCenter)
            alignment = rive.Alignment.bottomCenter;
        else if (this.alignment === Alignment.BottomRight)
            alignment = rive.Alignment.bottomRight;
        else
            alignment = rive.Alignment.center;
        this.cachedRuntimeAlignment = alignment;
        return alignment;
    };
    return Layout;
}());

// Runtime singleton; use getInstance to provide a callback that returns the
// Rive runtime
var RuntimeLoader = /** @class */ (function () {
    // Class is never instantiated
    function RuntimeLoader() {
    }
    // Loads the runtime
    RuntimeLoader.loadRuntime = function () {
        _rive_advanced_mjs__WEBPACK_IMPORTED_MODULE_0__["default"]({
            // Loads Wasm bundle
            locateFile: function () { return RuntimeLoader.wasmURL; },
        })
            .then(function (rive) {
            var _a;
            RuntimeLoader.runtime = rive;
            // Fire all the callbacks
            while (RuntimeLoader.callBackQueue.length > 0) {
                (_a = RuntimeLoader.callBackQueue.shift()) === null || _a === void 0 ? void 0 : _a(RuntimeLoader.runtime);
            }
        })
            .catch(function (error) {
            // Capture specific error details
            var errorDetails = {
                message: (error === null || error === void 0 ? void 0 : error.message) || "Unknown error",
                type: (error === null || error === void 0 ? void 0 : error.name) || "Error",
                // Some browsers may provide additional WebAssembly-specific details
                wasmError: error instanceof WebAssembly.CompileError ||
                    error instanceof WebAssembly.RuntimeError,
                originalError: error,
            };
            // Log detailed error for debugging
            console.debug("Rive WASM load error details:", errorDetails);
            // In case unpkg fails, or the wasm was not supported, we try to load the fallback module from jsdelivr.
            // This `rive_fallback.wasm` is compiled to support older architecture.
            // TODO: (Gordon): preemptively test browser support and load the correct wasm file. Then use jsdelvr only if unpkg fails.
            var backupJsdelivrUrl = "https://cdn.jsdelivr.net/npm/".concat(package_json__WEBPACK_IMPORTED_MODULE_1__.name, "@").concat(package_json__WEBPACK_IMPORTED_MODULE_1__.version, "/rive_fallback.wasm");
            if (RuntimeLoader.wasmURL.toLowerCase() !== backupJsdelivrUrl) {
                console.warn("Failed to load WASM from ".concat(RuntimeLoader.wasmURL, " (").concat(errorDetails.message, "), trying jsdelivr as a backup"));
                RuntimeLoader.setWasmUrl(backupJsdelivrUrl);
                RuntimeLoader.loadRuntime();
            }
            else {
                var errorMessage = [
                    "Could not load Rive WASM file from ".concat(RuntimeLoader.wasmURL, " or ").concat(backupJsdelivrUrl, "."),
                    "Possible reasons:",
                    "- Network connection is down",
                    "- WebAssembly is not supported in this environment",
                    "- The WASM file is corrupted or incompatible",
                    "\nError details:",
                    "- Type: ".concat(errorDetails.type),
                    "- Message: ".concat(errorDetails.message),
                    "- WebAssembly-specific error: ".concat(errorDetails.wasmError),
                    "\nTo resolve, you may need to:",
                    "1. Check your network connection",
                    "2. Set a new WASM source via RuntimeLoader.setWasmUrl()",
                    "3. Call RuntimeLoader.loadRuntime() again",
                ].join("\n");
                console.error(errorMessage);
            }
        });
    };
    // Provides a runtime instance via a callback
    RuntimeLoader.getInstance = function (callback) {
        // If it's not loading, start loading runtime
        if (!RuntimeLoader.isLoading) {
            RuntimeLoader.isLoading = true;
            RuntimeLoader.loadRuntime();
        }
        if (!RuntimeLoader.runtime) {
            RuntimeLoader.callBackQueue.push(callback);
        }
        else {
            callback(RuntimeLoader.runtime);
        }
    };
    // Provides a runtime instance via a promise
    RuntimeLoader.awaitInstance = function () {
        return new Promise(function (resolve) {
            return RuntimeLoader.getInstance(function (rive) { return resolve(rive); });
        });
    };
    // Manually sets the wasm url
    RuntimeLoader.setWasmUrl = function (url) {
        RuntimeLoader.wasmURL = url;
    };
    // Gets the current wasm url
    RuntimeLoader.getWasmUrl = function () {
        return RuntimeLoader.wasmURL;
    };
    // Flag to indicate that loading has started/completed
    RuntimeLoader.isLoading = false;
    // List of callbacks for the runtime that come in while loading
    RuntimeLoader.callBackQueue = [];
    // Path to the Wasm file; default path works for testing only;
    // if embedded wasm is used then this is never used.
    RuntimeLoader.wasmURL = "https://unpkg.com/".concat(package_json__WEBPACK_IMPORTED_MODULE_1__.name, "@").concat(package_json__WEBPACK_IMPORTED_MODULE_1__.version, "/rive.wasm");
    return RuntimeLoader;
}());

// #endregion
// #region state machines
var StateMachineInputType;
(function (StateMachineInputType) {
    StateMachineInputType[StateMachineInputType["Number"] = 56] = "Number";
    StateMachineInputType[StateMachineInputType["Trigger"] = 58] = "Trigger";
    StateMachineInputType[StateMachineInputType["Boolean"] = 59] = "Boolean";
})(StateMachineInputType || (StateMachineInputType = {}));
/**
 * An input for a state machine
 */
var StateMachineInput = /** @class */ (function () {
    function StateMachineInput(type, runtimeInput) {
        this.type = type;
        this.runtimeInput = runtimeInput;
    }
    Object.defineProperty(StateMachineInput.prototype, "name", {
        /**
         * Returns the name of the input
         */
        get: function () {
            return this.runtimeInput.name;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(StateMachineInput.prototype, "value", {
        /**
         * Returns the current value of the input
         */
        get: function () {
            return this.runtimeInput.value;
        },
        /**
         * Sets the value of the input
         */
        set: function (value) {
            this.runtimeInput.value = value;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Fires a trigger; does nothing on Number or Boolean input types
     */
    StateMachineInput.prototype.fire = function () {
        if (this.type === StateMachineInputType.Trigger) {
            this.runtimeInput.fire();
        }
    };
    /**
     * Deletes the input
     */
    StateMachineInput.prototype.delete = function () {
        this.runtimeInput = null;
    };
    return StateMachineInput;
}());

var RiveEventType;
(function (RiveEventType) {
    RiveEventType[RiveEventType["General"] = 128] = "General";
    RiveEventType[RiveEventType["OpenUrl"] = 131] = "OpenUrl";
})(RiveEventType || (RiveEventType = {}));
var BaseArtboard = /** @class */ (function () {
    function BaseArtboard(_isBindableArtboard) {
        this.isBindableArtboard = false;
        this.isBindableArtboard = _isBindableArtboard;
    }
    return BaseArtboard;
}());
var Artboard = /** @class */ (function (_super) {
    __extends(Artboard, _super);
    function Artboard(artboard, _file) {
        var _this = _super.call(this, false) || this;
        _this.nativeArtboard = artboard;
        _this.file = _file;
        return _this;
    }
    return Artboard;
}(BaseArtboard));
var BindableArtboard = /** @class */ (function (_super) {
    __extends(BindableArtboard, _super);
    function BindableArtboard(artboard) {
        var _this = _super.call(this, true) || this;
        _this.selfUnref = false;
        _this.nativeArtboard = artboard;
        return _this;
    }
    Object.defineProperty(BindableArtboard.prototype, "viewModel", {
        set: function (value) {
            this.nativeViewModel = value.nativeInstance;
        },
        enumerable: false,
        configurable: true
    });
    BindableArtboard.prototype.destroy = function () {
        var _a;
        if (this.selfUnref) {
            this.nativeArtboard.unref();
            (_a = this.nativeViewModel) === null || _a === void 0 ? void 0 : _a.unref();
        }
    };
    return BindableArtboard;
}(BaseArtboard));
var StateMachine = /** @class */ (function () {
    /**
     * @constructor
     * @param stateMachine runtime state machine object
     * @param instance runtime state machine instance object
     */
    function StateMachine(stateMachine, runtime, playing, artboard) {
        this.stateMachine = stateMachine;
        this.playing = playing;
        this.artboard = artboard;
        /**
         * Caches the inputs from the runtime
         */
        this.inputs = [];
        this.instance = new runtime.StateMachineInstance(stateMachine, artboard);
        this.initInputs(runtime);
    }
    Object.defineProperty(StateMachine.prototype, "name", {
        get: function () {
            return this.stateMachine.name;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(StateMachine.prototype, "statesChanged", {
        /**
         * Returns a list of state names that have changed on this frame
         */
        get: function () {
            var names = [];
            for (var i = 0; i < this.instance.stateChangedCount(); i++) {
                names.push(this.instance.stateChangedNameByIndex(i));
            }
            return names;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Advances the state machine instance by a given time.
     * @param time - the time to advance the animation by in seconds
     */
    StateMachine.prototype.advance = function (time) {
        this.instance.advance(time);
    };
    /**
     * Advances the state machine instance by a given time and apply changes to artboard.
     * @param time - the time to advance the animation by in seconds
     */
    StateMachine.prototype.advanceAndApply = function (time) {
        this.instance.advanceAndApply(time);
    };
    /**
     * Returns the number of events reported from the last advance call
     * @returns Number of events reported
     */
    StateMachine.prototype.reportedEventCount = function () {
        return this.instance.reportedEventCount();
    };
    /**
     * Returns a RiveEvent object emitted from the last advance call at the given index
     * of a list of potentially multiple events. If an event at the index is not found,
     * undefined is returned.
     * @param i index of the event reported in a list of potentially multiple events
     * @returns RiveEvent or extended RiveEvent object returned, or undefined
     */
    StateMachine.prototype.reportedEventAt = function (i) {
        return this.instance.reportedEventAt(i);
    };
    /**
     * Fetches references to the state machine's inputs and caches them
     * @param runtime an instance of the runtime; needed for the SMIInput types
     */
    StateMachine.prototype.initInputs = function (runtime) {
        // Fetch the inputs from the runtime if we don't have them
        for (var i = 0; i < this.instance.inputCount(); i++) {
            var input = this.instance.input(i);
            this.inputs.push(this.mapRuntimeInput(input, runtime));
        }
    };
    /**
     * Maps a runtime input to it's appropriate type
     * @param input
     */
    StateMachine.prototype.mapRuntimeInput = function (input, runtime) {
        if (input.type === runtime.SMIInput.bool) {
            return new StateMachineInput(StateMachineInputType.Boolean, input.asBool());
        }
        else if (input.type === runtime.SMIInput.number) {
            return new StateMachineInput(StateMachineInputType.Number, input.asNumber());
        }
        else if (input.type === runtime.SMIInput.trigger) {
            return new StateMachineInput(StateMachineInputType.Trigger, input.asTrigger());
        }
    };
    /**
     * Deletes the backing Wasm state machine instance; once this is called, this
     * state machine is no more.
     */
    StateMachine.prototype.cleanup = function () {
        this.inputs.forEach(function (input) {
            input.delete();
        });
        this.inputs.length = 0;
        this.instance.delete();
    };
    StateMachine.prototype.bindViewModelInstance = function (viewModelInstance) {
        if (viewModelInstance.runtimeInstance != null) {
            this.instance.bindViewModelInstance(viewModelInstance.runtimeInstance);
        }
    };
    return StateMachine;
}());
// #endregion
// #region animator
/**
 * Manages animation
 */
var Animator = /** @class */ (function () {
    /**
     * Constructs a new animator
     * @constructor
     * @param runtime Rive runtime; needed to instance animations & state machines
     * @param artboard the artboard that holds all animations and state machines
     * @param animations optional list of animations
     * @param stateMachines optional list of state machines
     */
    function Animator(runtime, artboard, eventManager, animations, stateMachines) {
        if (animations === void 0) { animations = []; }
        if (stateMachines === void 0) { stateMachines = []; }
        this.runtime = runtime;
        this.artboard = artboard;
        this.eventManager = eventManager;
        this.animations = animations;
        this.stateMachines = stateMachines;
    }
    /**
     * Adds animations and state machines by their names. If names are shared
     * between animations & state machines, then the first one found will be
     * created. Best not to use the same names for these in your Rive file.
     * @param animatable the name(s) of animations and state machines to add
     * @returns a list of names of the playing animations and state machines
     */
    Animator.prototype.add = function (animatables, playing, fireEvent) {
        if (fireEvent === void 0) { fireEvent = true; }
        animatables = mapToStringArray(animatables);
        // If animatables is empty, play or pause everything
        if (animatables.length === 0) {
            this.animations.forEach(function (a) { return (a.playing = playing); });
            this.stateMachines.forEach(function (m) { return (m.playing = playing); });
        }
        else {
            // Play/pause already instanced items, or create new instances
            var instancedAnimationNames = this.animations.map(function (a) { return a.name; });
            var instancedMachineNames = this.stateMachines.map(function (m) { return m.name; });
            for (var i = 0; i < animatables.length; i++) {
                var aIndex = instancedAnimationNames.indexOf(animatables[i]);
                var mIndex = instancedMachineNames.indexOf(animatables[i]);
                if (aIndex >= 0 || mIndex >= 0) {
                    if (aIndex >= 0) {
                        // Animation is instanced, play/pause it
                        this.animations[aIndex].playing = playing;
                    }
                    else {
                        // State machine is instanced, play/pause it
                        this.stateMachines[mIndex].playing = playing;
                    }
                }
                else {
                    // Try to create a new animation instance
                    var anim = this.artboard.animationByName(animatables[i]);
                    if (anim) {
                        var newAnimation = new _animation__WEBPACK_IMPORTED_MODULE_2__.Animation(anim, this.artboard, this.runtime, playing);
                        // Display the first frame of the specified animation
                        newAnimation.advance(0);
                        newAnimation.apply(1.0);
                        this.animations.push(newAnimation);
                    }
                    else {
                        // Try to create a new state machine instance
                        var sm = this.artboard.stateMachineByName(animatables[i]);
                        if (sm) {
                            var newStateMachine = new StateMachine(sm, this.runtime, playing, this.artboard);
                            this.stateMachines.push(newStateMachine);
                        }
                    }
                }
            }
        }
        // Fire play/paused events for animations
        if (fireEvent) {
            if (playing) {
                this.eventManager.fire({
                    type: EventType.Play,
                    data: this.playing,
                });
            }
            else {
                this.eventManager.fire({
                    type: EventType.Pause,
                    data: this.paused,
                });
            }
        }
        return playing ? this.playing : this.paused;
    };
    /**
     * Adds linear animations by their names.
     * @param animatables the name(s) of animations to add
     * @param playing whether animations should play on instantiation
     */
    Animator.prototype.initLinearAnimations = function (animatables, playing) {
        // Play/pause already instanced items, or create new instances
        // This validation is kept to maintain compatibility with current behavior.
        // But given that it this is called during artboard initialization
        // it should probably be safe to remove.
        var instancedAnimationNames = this.animations.map(function (a) { return a.name; });
        for (var i = 0; i < animatables.length; i++) {
            var aIndex = instancedAnimationNames.indexOf(animatables[i]);
            if (aIndex >= 0) {
                this.animations[aIndex].playing = playing;
            }
            else {
                // Try to create a new animation instance
                var anim = this.artboard.animationByName(animatables[i]);
                if (anim) {
                    var newAnimation = new _animation__WEBPACK_IMPORTED_MODULE_2__.Animation(anim, this.artboard, this.runtime, playing);
                    // Display the first frame of the specified animation
                    newAnimation.advance(0);
                    newAnimation.apply(1.0);
                    this.animations.push(newAnimation);
                }
                else {
                    console.error("Animation with name ".concat(animatables[i], " not found."));
                }
            }
        }
    };
    /**
     * Adds state machines by their names.
     * @param animatables the name(s) of state machines to add
     * @param playing whether state machines should play on instantiation
     */
    Animator.prototype.initStateMachines = function (animatables, playing) {
        // Play/pause already instanced items, or create new instances
        // This validation is kept to maintain compatibility with current behavior.
        // But given that it this is called during artboard initialization
        // it should probably be safe to remove.
        var instancedStateMachineNames = this.stateMachines.map(function (a) { return a.name; });
        for (var i = 0; i < animatables.length; i++) {
            var aIndex = instancedStateMachineNames.indexOf(animatables[i]);
            if (aIndex >= 0) {
                this.stateMachines[aIndex].playing = playing;
            }
            else {
                // Try to create a new state machine instance
                var sm = this.artboard.stateMachineByName(animatables[i]);
                if (sm) {
                    var newStateMachine = new StateMachine(sm, this.runtime, playing, this.artboard);
                    this.stateMachines.push(newStateMachine);
                }
                else {
                    console.warn("State Machine with name ".concat(animatables[i], " not found."));
                    // In order to maintain compatibility with current behavior, if a state machine is not found
                    // we look for an animation with the same name
                    this.initLinearAnimations([animatables[i]], playing);
                }
            }
        }
    };
    /**
     * Play the named animations/state machines
     * @param animatables the names of the animations/machines to play; plays all if empty
     * @returns a list of the playing items
     */
    Animator.prototype.play = function (animatables) {
        return this.add(animatables, true);
    };
    /**
     * Advance state machines if they are paused after initialization
     */
    Animator.prototype.advanceIfPaused = function () {
        this.stateMachines.forEach(function (sm) {
            if (!sm.playing) {
                sm.advanceAndApply(0);
            }
        });
    };
    /**
     * Pauses named animations and state machines, or everything if nothing is
     * specified
     * @param animatables names of the animations and state machines to pause
     * @returns a list of names of the animations and state machines paused
     */
    Animator.prototype.pause = function (animatables) {
        return this.add(animatables, false);
    };
    /**
     * Set time of named animations
     * @param animations names of the animations to scrub
     * @param value time scrub value, a floating point number to which the playhead is jumped
     * @returns a list of names of the animations that were scrubbed
     */
    Animator.prototype.scrub = function (animatables, value) {
        var forScrubbing = this.animations.filter(function (a) {
            return animatables.includes(a.name);
        });
        forScrubbing.forEach(function (a) { return (a.scrubTo = value); });
        return forScrubbing.map(function (a) { return a.name; });
    };
    Object.defineProperty(Animator.prototype, "playing", {
        /**
         * Returns a list of names of all animations and state machines currently
         * playing
         */
        get: function () {
            return this.animations
                .filter(function (a) { return a.playing; })
                .map(function (a) { return a.name; })
                .concat(this.stateMachines.filter(function (m) { return m.playing; }).map(function (m) { return m.name; }));
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Animator.prototype, "paused", {
        /**
         * Returns a list of names of all animations and state machines currently
         * paused
         */
        get: function () {
            return this.animations
                .filter(function (a) { return !a.playing; })
                .map(function (a) { return a.name; })
                .concat(this.stateMachines.filter(function (m) { return !m.playing; }).map(function (m) { return m.name; }));
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Stops and removes all named animations and state machines
     * @param animatables animations and state machines to remove
     * @returns a list of names of removed items
     */
    Animator.prototype.stop = function (animatables) {
        var _this = this;
        animatables = mapToStringArray(animatables);
        // If nothing's specified, wipe them out, all of them
        var removedNames = [];
        // Stop everything
        if (animatables.length === 0) {
            removedNames = this.animations
                .map(function (a) { return a.name; })
                .concat(this.stateMachines.map(function (m) { return m.name; }));
            // Clean up before emptying the arrays
            this.animations.forEach(function (a) { return a.cleanup(); });
            this.stateMachines.forEach(function (m) { return m.cleanup(); });
            // Empty out the arrays
            this.animations.splice(0, this.animations.length);
            this.stateMachines.splice(0, this.stateMachines.length);
        }
        else {
            // Remove only the named animations/state machines
            var animationsToRemove = this.animations.filter(function (a) {
                return animatables.includes(a.name);
            });
            animationsToRemove.forEach(function (a) {
                a.cleanup();
                _this.animations.splice(_this.animations.indexOf(a), 1);
            });
            var machinesToRemove = this.stateMachines.filter(function (m) {
                return animatables.includes(m.name);
            });
            machinesToRemove.forEach(function (m) {
                m.cleanup();
                _this.stateMachines.splice(_this.stateMachines.indexOf(m), 1);
            });
            removedNames = animationsToRemove
                .map(function (a) { return a.name; })
                .concat(machinesToRemove.map(function (m) { return m.name; }));
        }
        this.eventManager.fire({
            type: EventType.Stop,
            data: removedNames,
        });
        // Return the list of animations removed
        return removedNames;
    };
    Object.defineProperty(Animator.prototype, "isPlaying", {
        /**
         * Returns true if at least one animation is active
         */
        get: function () {
            return (this.animations.reduce(function (acc, curr) { return acc || curr.playing; }, false) ||
                this.stateMachines.reduce(function (acc, curr) { return acc || curr.playing; }, false));
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Animator.prototype, "isPaused", {
        /**
         * Returns true if all animations are paused and there's at least one animation
         */
        get: function () {
            return (!this.isPlaying &&
                (this.animations.length > 0 || this.stateMachines.length > 0));
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Animator.prototype, "isStopped", {
        /**
         * Returns true if there are no playing or paused animations/state machines
         */
        get: function () {
            return this.animations.length === 0 && this.stateMachines.length === 0;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * If there are no animations or state machines, add the first one found
     * @returns the name of the animation or state machine instanced
     */
    Animator.prototype.atLeastOne = function (playing, fireEvent) {
        if (fireEvent === void 0) { fireEvent = true; }
        var instancedName;
        if (this.animations.length === 0 && this.stateMachines.length === 0) {
            if (this.artboard.animationCount() > 0) {
                // Add the first animation
                this.add([(instancedName = this.artboard.animationByIndex(0).name)], playing, fireEvent);
            }
            else if (this.artboard.stateMachineCount() > 0) {
                // Add the first state machine
                this.add([(instancedName = this.artboard.stateMachineByIndex(0).name)], playing, fireEvent);
            }
        }
        return instancedName;
    };
    /**
     * Checks if any animations have looped and if so, fire the appropriate event
     */
    Animator.prototype.handleLooping = function () {
        for (var _i = 0, _a = this.animations.filter(function (a) { return a.playing; }); _i < _a.length; _i++) {
            var animation = _a[_i];
            // Emit if the animation looped
            if (animation.loopValue === 0 && animation.loopCount) {
                animation.loopCount = 0;
                // This is a one-shot; if it has ended, delete the instance
                this.stop(animation.name);
            }
            else if (animation.loopValue === 1 && animation.loopCount) {
                this.eventManager.fire({
                    type: EventType.Loop,
                    data: { animation: animation.name, type: LoopType.Loop },
                });
                animation.loopCount = 0;
            }
            // Wasm indicates a loop at each time the animation
            // changes direction, so a full loop/lap occurs every
            // two loop counts
            else if (animation.loopValue === 2 && animation.loopCount > 1) {
                this.eventManager.fire({
                    type: EventType.Loop,
                    data: { animation: animation.name, type: LoopType.PingPong },
                });
                animation.loopCount = 0;
            }
        }
    };
    /**
     * Checks if states have changed in state machines and fires a statechange
     * event
     */
    Animator.prototype.handleStateChanges = function () {
        var statesChanged = [];
        for (var _i = 0, _a = this.stateMachines.filter(function (sm) { return sm.playing; }); _i < _a.length; _i++) {
            var stateMachine = _a[_i];
            statesChanged.push.apply(statesChanged, stateMachine.statesChanged);
        }
        if (statesChanged.length > 0) {
            this.eventManager.fire({
                type: EventType.StateChange,
                data: statesChanged,
            });
        }
    };
    Animator.prototype.handleAdvancing = function (time) {
        this.eventManager.fire({
            type: EventType.Advance,
            data: time,
        });
    };
    return Animator;
}());
// #endregion
// #region events
/**
 * Supported event types triggered in Rive
 */
var EventType;
(function (EventType) {
    EventType["Load"] = "load";
    EventType["LoadError"] = "loaderror";
    EventType["Play"] = "play";
    EventType["Pause"] = "pause";
    EventType["Stop"] = "stop";
    EventType["Loop"] = "loop";
    EventType["Draw"] = "draw";
    EventType["Advance"] = "advance";
    EventType["StateChange"] = "statechange";
    EventType["RiveEvent"] = "riveevent";
    EventType["AudioStatusChange"] = "audiostatuschange";
})(EventType || (EventType = {}));
/**
 * Looping types: one-shot, loop, and ping-pong
 */
var LoopType;
(function (LoopType) {
    LoopType["OneShot"] = "oneshot";
    LoopType["Loop"] = "loop";
    LoopType["PingPong"] = "pingpong";
})(LoopType || (LoopType = {}));
// Manages Rive events and listeners
var EventManager = /** @class */ (function () {
    function EventManager(listeners) {
        if (listeners === void 0) { listeners = []; }
        this.listeners = listeners;
    }
    // Gets listeners of specified type
    EventManager.prototype.getListeners = function (type) {
        return this.listeners.filter(function (e) { return e.type === type; });
    };
    // Adds a listener
    EventManager.prototype.add = function (listener) {
        if (!this.listeners.includes(listener)) {
            this.listeners.push(listener);
        }
    };
    /**
     * Removes a listener
     * @param listener the listener with the callback to be removed
     */
    EventManager.prototype.remove = function (listener) {
        // We can't simply look for the listener as it'll be a different instance to
        // one originally subscribed. Find all the listeners of the right type and
        // then check their callbacks which should match.
        for (var i = 0; i < this.listeners.length; i++) {
            var currentListener = this.listeners[i];
            if (currentListener.type === listener.type) {
                if (currentListener.callback === listener.callback) {
                    this.listeners.splice(i, 1);
                    break;
                }
            }
        }
    };
    /**
     * Clears all listeners of specified type, or every listener if no type is
     * specified
     * @param type the type of listeners to clear, or all listeners if not
     * specified
     */
    EventManager.prototype.removeAll = function (type) {
        var _this = this;
        if (!type) {
            this.listeners.splice(0, this.listeners.length);
        }
        else {
            this.listeners
                .filter(function (l) { return l.type === type; })
                .forEach(function (l) { return _this.remove(l); });
        }
    };
    // Fires an event
    EventManager.prototype.fire = function (event) {
        var eventListeners = this.getListeners(event.type);
        eventListeners.forEach(function (listener) { return listener.callback(event); });
    };
    return EventManager;
}());
// Manages a queue of tasks
var TaskQueueManager = /** @class */ (function () {
    function TaskQueueManager(eventManager) {
        this.eventManager = eventManager;
        this.queue = [];
    }
    // Adds a task top the queue
    TaskQueueManager.prototype.add = function (task) {
        this.queue.push(task);
    };
    // Processes all tasks in the queue
    TaskQueueManager.prototype.process = function () {
        while (this.queue.length > 0) {
            var task = this.queue.shift();
            if (task === null || task === void 0 ? void 0 : task.action) {
                task.action();
            }
            if (task === null || task === void 0 ? void 0 : task.event) {
                this.eventManager.fire(task.event);
            }
        }
    };
    return TaskQueueManager;
}());
// #endregion
// #region Audio
var SystemAudioStatus;
(function (SystemAudioStatus) {
    SystemAudioStatus[SystemAudioStatus["AVAILABLE"] = 0] = "AVAILABLE";
    SystemAudioStatus[SystemAudioStatus["UNAVAILABLE"] = 1] = "UNAVAILABLE";
})(SystemAudioStatus || (SystemAudioStatus = {}));
// Class to handle audio context availability and status changes
var AudioManager = /** @class */ (function (_super) {
    __extends(AudioManager, _super);
    function AudioManager() {
        var _this = _super !== null && _super.apply(this, arguments) || this;
        _this._started = false;
        _this._enabled = false;
        _this._status = SystemAudioStatus.UNAVAILABLE;
        return _this;
    }
    AudioManager.prototype.delay = function (time) {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, new Promise(function (resolve) { return setTimeout(resolve, time); })];
            });
        });
    };
    AudioManager.prototype.timeout = function () {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, new Promise(function (_, reject) { return setTimeout(reject, 50); })];
            });
        });
    };
    // Alerts animations on status changes and removes the listeners to avoid alerting twice.
    AudioManager.prototype.reportToListeners = function () {
        this.fire({ type: EventType.AudioStatusChange });
        this.removeAll();
    };
    /**
     * The audio context has been resolved.
     * Alert any listeners that we can now play audio.
     * Rive will now play audio at the configured volume.
     */
    AudioManager.prototype.enableAudio = function () {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                if (!this._enabled) {
                    this._enabled = true;
                    this._status = SystemAudioStatus.AVAILABLE;
                    this.reportToListeners();
                }
                return [2 /*return*/];
            });
        });
    };
    /**
     * Check if we are able to play audio.
     *
     * We currently check the audio context, when resume() returns before a timeout we know that the
     * audio context is running and we can enable audio.
     */
    AudioManager.prototype.testAudio = function () {
        return __awaiter(this, void 0, void 0, function () {
            var _a;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        if (!(this._status === SystemAudioStatus.UNAVAILABLE &&
                            this._audioContext !== null)) return [3 /*break*/, 4];
                        _b.label = 1;
                    case 1:
                        _b.trys.push([1, 3, , 4]);
                        return [4 /*yield*/, Promise.race([this._audioContext.resume(), this.timeout()])];
                    case 2:
                        _b.sent();
                        this.enableAudio();
                        return [3 /*break*/, 4];
                    case 3:
                        _a = _b.sent();
                        return [3 /*break*/, 4];
                    case 4: return [2 /*return*/];
                }
            });
        });
    };
    /**
     * Establish audio for use with rive.
     * We both test if we can use audio intermittently and listen for user interaction.
     * The aim is to enable audio playback as soon as the browser allows this.
     */
    AudioManager.prototype._establishAudio = function () {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (!!this._started) return [3 /*break*/, 5];
                        this._started = true;
                        if (!(typeof window == "undefined")) return [3 /*break*/, 1];
                        this.enableAudio();
                        return [3 /*break*/, 5];
                    case 1:
                        this._audioContext = new AudioContext();
                        this.listenForUserAction();
                        _a.label = 2;
                    case 2:
                        if (!(this._status === SystemAudioStatus.UNAVAILABLE)) return [3 /*break*/, 5];
                        return [4 /*yield*/, this.testAudio()];
                    case 3:
                        _a.sent();
                        return [4 /*yield*/, this.delay(1000)];
                    case 4:
                        _a.sent();
                        return [3 /*break*/, 2];
                    case 5: return [2 /*return*/];
                }
            });
        });
    };
    AudioManager.prototype.listenForUserAction = function () {
        var _this = this;
        // NOTE: AudioContexts are ready immediately if requested in a ui callback
        // we *could* re request one in this listener.
        var _clickListener = function () { return __awaiter(_this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                // note this has "better" results than calling `await this.testAudio()`
                // as we force audio to be enabled in the current thread, rather than chancing
                // the thread to be passed over for some other async context
                this.enableAudio();
                return [2 /*return*/];
            });
        }); };
        // NOTE: we should test this on mobile/pads
        document.addEventListener("pointerdown", _clickListener, {
            once: true,
        });
    };
    /**
     * Establish the audio context for rive, this lets rive know that we can play audio.
     */
    AudioManager.prototype.establishAudio = function () {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                this._establishAudio();
                return [2 /*return*/];
            });
        });
    };
    Object.defineProperty(AudioManager.prototype, "systemVolume", {
        get: function () {
            if (this._status === SystemAudioStatus.UNAVAILABLE) {
                // We do an immediate test to avoid depending on the delay of the running test
                this.testAudio();
                return 0;
            }
            return 1;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(AudioManager.prototype, "status", {
        get: function () {
            return this._status;
        },
        enumerable: false,
        configurable: true
    });
    return AudioManager;
}(EventManager));
var audioManager = new AudioManager();
var FakeResizeObserver = /** @class */ (function () {
    function FakeResizeObserver() {
    }
    FakeResizeObserver.prototype.observe = function () { };
    FakeResizeObserver.prototype.unobserve = function () { };
    FakeResizeObserver.prototype.disconnect = function () { };
    return FakeResizeObserver;
}());
var MyResizeObserver = globalThis.ResizeObserver || FakeResizeObserver;
/**
 * This class takes care of any observers that will be attached to an animation.
 * It should be treated as a singleton because observers are much more performant
 * when used for observing multiple elements by a single instance.
 */
var ObjectObservers = /** @class */ (function () {
    function ObjectObservers() {
        var _this = this;
        this._elementsMap = new Map();
        /**
         * Resize observers trigger both when the element changes its size and also when the
         * element is added or removed from the document.
         */
        this._onObservedEntry = function (entry) {
            var observed = _this._elementsMap.get(entry.target);
            if (observed !== null) {
                observed.onResize(entry.target.clientWidth == 0 || entry.target.clientHeight == 0);
            }
            else {
                _this._resizeObserver.unobserve(entry.target);
            }
        };
        this._onObserved = function (entries) {
            entries.forEach(_this._onObservedEntry);
        };
        this._resizeObserver = new MyResizeObserver(this._onObserved);
    }
    // Adds an observable element
    ObjectObservers.prototype.add = function (element, onResize) {
        var observed = {
            onResize: onResize,
            element: element,
        };
        this._elementsMap.set(element, observed);
        this._resizeObserver.observe(element);
        return observed;
    };
    // Removes an observable element
    ObjectObservers.prototype.remove = function (observed) {
        this._resizeObserver.unobserve(observed.element);
        this._elementsMap.delete(observed.element);
    };
    return ObjectObservers;
}());
var observers = new ObjectObservers();
var RiveFile = /** @class */ (function () {
    function RiveFile(params) {
        // Allow the runtime to automatically load assets hosted in Rive's runtime.
        this.enableRiveAssetCDN = true;
        this.referenceCount = 0;
        this.destroyed = false;
        this.selfUnref = false;
        this.bindableArtboards = [];
        this.src = params.src;
        this.buffer = params.buffer;
        if (params.assetLoader)
            this.assetLoader = params.assetLoader;
        this.enableRiveAssetCDN =
            typeof params.enableRiveAssetCDN == "boolean"
                ? params.enableRiveAssetCDN
                : true;
        // New event management system
        this.eventManager = new EventManager();
        if (params.onLoad)
            this.on(EventType.Load, params.onLoad);
        if (params.onLoadError)
            this.on(EventType.LoadError, params.onLoadError);
    }
    RiveFile.prototype.releaseFile = function () {
        var _a;
        if (this.selfUnref) {
            (_a = this.file) === null || _a === void 0 ? void 0 : _a.unref();
        }
        this.file = null;
    };
    RiveFile.prototype.releaseBindableArtboards = function () {
        this.bindableArtboards.forEach(function (bindableArtboard) {
            return bindableArtboard.destroy();
        });
    };
    RiveFile.prototype.initData = function () {
        return __awaiter(this, void 0, void 0, function () {
            var _a, loader, loaderWrapper, _b, fileFinalizer;
            return __generator(this, function (_c) {
                switch (_c.label) {
                    case 0:
                        if (!this.src) return [3 /*break*/, 2];
                        _a = this;
                        return [4 /*yield*/, loadRiveFile(this.src)];
                    case 1:
                        _a.buffer = _c.sent();
                        _c.label = 2;
                    case 2:
                        if (this.destroyed) {
                            return [2 /*return*/];
                        }
                        if (this.assetLoader) {
                            loaderWrapper = new _utils__WEBPACK_IMPORTED_MODULE_3__.CustomFileAssetLoaderWrapper(this.runtime, this.assetLoader);
                            loader = loaderWrapper.assetLoader;
                        }
                        // Load the Rive file
                        _b = this;
                        return [4 /*yield*/, this.runtime.load(new Uint8Array(this.buffer), loader, this.enableRiveAssetCDN)];
                    case 3:
                        // Load the Rive file
                        _b.file = _c.sent();
                        fileFinalizer = new _utils__WEBPACK_IMPORTED_MODULE_3__.FileFinalizer(this.file);
                        _utils__WEBPACK_IMPORTED_MODULE_3__.finalizationRegistry.register(this, fileFinalizer);
                        if (this.destroyed) {
                            this.releaseFile();
                            return [2 /*return*/];
                        }
                        if (this.file !== null) {
                            this.eventManager.fire({
                                type: EventType.Load,
                                data: this,
                            });
                        }
                        else {
                            this.fireLoadError(RiveFile.fileLoadErrorMessage);
                        }
                        return [2 /*return*/];
                }
            });
        });
    };
    RiveFile.prototype.init = function () {
        return __awaiter(this, void 0, void 0, function () {
            var _a, error_1;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        // If no source file url specified, it's a bust
                        if (!this.src && !this.buffer) {
                            this.fireLoadError(RiveFile.missingErrorMessage);
                            return [2 /*return*/];
                        }
                        _b.label = 1;
                    case 1:
                        _b.trys.push([1, 4, , 5]);
                        _a = this;
                        return [4 /*yield*/, RuntimeLoader.awaitInstance()];
                    case 2:
                        _a.runtime = _b.sent();
                        if (this.destroyed) {
                            return [2 /*return*/];
                        }
                        return [4 /*yield*/, this.initData()];
                    case 3:
                        _b.sent();
                        return [3 /*break*/, 5];
                    case 4:
                        error_1 = _b.sent();
                        this.fireLoadError(error_1 instanceof Error ? error_1.message : RiveFile.fileLoadErrorMessage);
                        return [3 /*break*/, 5];
                    case 5: return [2 /*return*/];
                }
            });
        });
    };
    RiveFile.prototype.fireLoadError = function (message) {
        this.eventManager.fire({
            type: EventType.LoadError,
            data: message,
        });
        throw new Error(message);
    };
    /**
     * Subscribe to Rive-generated events
     * @param type the type of event to subscribe to
     * @param callback callback to fire when the event occurs
     */
    RiveFile.prototype.on = function (type, callback) {
        this.eventManager.add({
            type: type,
            callback: callback,
        });
    };
    /**
     * Unsubscribes from a Rive-generated event
     * @param type the type of event to unsubscribe from
     * @param callback the callback to unsubscribe
     */
    RiveFile.prototype.off = function (type, callback) {
        this.eventManager.remove({
            type: type,
            callback: callback,
        });
    };
    RiveFile.prototype.cleanup = function () {
        this.referenceCount -= 1;
        if (this.referenceCount <= 0) {
            this.removeAllRiveEventListeners();
            this.releaseFile();
            this.releaseBindableArtboards();
            this.destroyed = true;
        }
    };
    /**
     * Unsubscribes all Rive listeners from an event type, or everything if no type is
     * given
     * @param type the type of event to unsubscribe from, or all types if
     * undefined
     */
    RiveFile.prototype.removeAllRiveEventListeners = function (type) {
        this.eventManager.removeAll(type);
    };
    RiveFile.prototype.getInstance = function () {
        if (this.file !== null) {
            this.referenceCount += 1;
            return this.file;
        }
    };
    RiveFile.prototype.destroyIfUnused = function () {
        if (this.referenceCount <= 0) {
            this.cleanup();
        }
    };
    RiveFile.prototype.createBindableArtboard = function (nativeBindableArtboard) {
        if (nativeBindableArtboard != null) {
            var bindableArtboard = new BindableArtboard(nativeBindableArtboard);
            (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(bindableArtboard, bindableArtboard.nativeArtboard);
            this.bindableArtboards.push(bindableArtboard);
            return bindableArtboard;
        }
        return null;
    };
    /**
     * @deprecated This function is deprecated. For better stability and memory management
     * use `getBindableArtboard()` instead.
     * @param {string} name - The name of the artboard.
     * @returns {Artboard} The artboard to bind to.
     */
    RiveFile.prototype.getArtboard = function (name) {
        var nativeArtboard = this.file.artboardByName(name);
        if (nativeArtboard != null) {
            return new Artboard(nativeArtboard, this);
        }
    };
    RiveFile.prototype.getBindableArtboard = function (name) {
        var nativeArtboard = this.file.bindableArtboardByName(name);
        return this.createBindableArtboard(nativeArtboard);
    };
    RiveFile.prototype.getDefaultBindableArtboard = function () {
        var nativeArtboard = this.file.bindableArtboardDefault();
        return this.createBindableArtboard(nativeArtboard);
    };
    RiveFile.prototype.internalBindableArtboardFromArtboard = function (artboard) {
        var nativeBindableArtboard = this.file.internalBindableArtboardFromArtboard(artboard);
        return this.createBindableArtboard(nativeBindableArtboard);
    };
    RiveFile.prototype.viewModelByName = function (name) {
        var viewModel = this.file.viewModelByName(name);
        if (viewModel !== null) {
            return new ViewModel(viewModel);
        }
        return null;
    };
    // Error message for missing source or buffer
    RiveFile.missingErrorMessage = "Rive source file or data buffer required";
    // Error message for file load error
    RiveFile.fileLoadErrorMessage = "The file failed to load";
    return RiveFile;
}());

var Rive = /** @class */ (function () {
    function Rive(params) {
        var _this = this;
        var _a, _b;
        // Tracks if a Rive file is loaded
        this.loaded = false;
        // Tracks if a Rive file is destroyed
        this.destroyed = false;
        // Reference of an object that handles any observers for the animation
        this._observed = null;
        /**
         * Tracks if a Rive file is loaded; we need this in addition to loaded as some
         * commands (e.g. contents) can be called as soon as the file is loaded.
         * However, playback commands need to be queued and run in order once initial
         * animations and autoplay has been sorted out. This applies to play, pause,
         * and start.
         */
        this.readyForPlaying = false;
        // Runtime artboard
        this.artboard = null;
        // place to clear up event listeners
        this.eventCleanup = null;
        this.shouldDisableRiveListeners = false;
        this.automaticallyHandleEvents = false;
        this.dispatchPointerExit = true;
        // Allow all pointers to be passed to the runtime
        this.enableMultiTouch = false;
        // Allow the runtime to automatically load assets hosted in Rive's runtime.
        this.enableRiveAssetCDN = true;
        // Keep a local value of the set volume to update it asynchronously
        this._volume = 1;
        // Keep a local value of the set width to update it asynchronously
        this._artboardWidth = undefined;
        // Keep a local value of the set height to update it asynchronously
        this._artboardHeight = undefined;
        // Keep a local value of the device pixel ratio used in rendering and canvas/artboard resizing
        this._devicePixelRatioUsed = 1;
        // Whether the canvas element's size is 0
        this._hasZeroSize = false;
        // Whether a draw operation needs to be forced
        this._needsRedraw = false;
        // Canvas width and height. Values are lazily updated so they might
        // not be in sync with current canvas size.
        this._currentCanvasWidth = 0;
        this._currentCanvasHeight = 0;
        // Audio event listener
        this._audioEventListener = null;
        // draw method bound to the class
        this._boundDraw = null;
        this._viewModelInstance = null;
        this._dataEnums = null;
        this.drawOptimization = DrawOptimizationOptions.DrawOnChanged;
        // Durations to generate a frame for the last second. Used for performance profiling.
        this.durations = [];
        this.frameTimes = [];
        this.frameCount = 0;
        this.isTouchScrollEnabled = false;
        this.onCanvasResize = function (hasZeroSize) {
            var toggledDisplay = _this._hasZeroSize !== hasZeroSize;
            _this._hasZeroSize = hasZeroSize;
            if (!hasZeroSize) {
                if (toggledDisplay) {
                    _this.resizeDrawingSurfaceToCanvas();
                }
            }
            else if (!_this._layout.maxX || !_this._layout.maxY) {
                _this.resizeToCanvas();
            }
        };
        /**
         * Used be draw to track when a second of active rendering time has passed.
         * Used for debugging purposes
         */
        this.renderSecondTimer = 0;
        this._boundDraw = this.draw.bind(this);
        this.canvas = params.canvas;
        if (params.canvas.constructor === HTMLCanvasElement) {
            this._observed = observers.add(this.canvas, this.onCanvasResize);
        }
        this._currentCanvasWidth = this.canvas.width;
        this._currentCanvasHeight = this.canvas.height;
        this.src = params.src;
        this.buffer = params.buffer;
        this.riveFile = params.riveFile;
        this.layout = (_a = params.layout) !== null && _a !== void 0 ? _a : new Layout();
        this.shouldDisableRiveListeners = !!params.shouldDisableRiveListeners;
        this.isTouchScrollEnabled = !!params.isTouchScrollEnabled;
        this.automaticallyHandleEvents = !!params.automaticallyHandleEvents;
        this.dispatchPointerExit =
            params.dispatchPointerExit === false
                ? params.dispatchPointerExit
                : this.dispatchPointerExit;
        this.enableMultiTouch = !!params.enableMultiTouch;
        this.drawOptimization = (_b = params.drawingOptions) !== null && _b !== void 0 ? _b : this.drawOptimization;
        this.enableRiveAssetCDN =
            params.enableRiveAssetCDN === undefined
                ? true
                : params.enableRiveAssetCDN;
        // New event management system
        this.eventManager = new EventManager();
        if (params.onLoad)
            this.on(EventType.Load, params.onLoad);
        if (params.onLoadError)
            this.on(EventType.LoadError, params.onLoadError);
        if (params.onPlay)
            this.on(EventType.Play, params.onPlay);
        if (params.onPause)
            this.on(EventType.Pause, params.onPause);
        if (params.onStop)
            this.on(EventType.Stop, params.onStop);
        if (params.onLoop)
            this.on(EventType.Loop, params.onLoop);
        if (params.onStateChange)
            this.on(EventType.StateChange, params.onStateChange);
        if (params.onAdvance)
            this.on(EventType.Advance, params.onAdvance);
        /**
         * @deprecated Use camelCase'd versions instead.
         */
        if (params.onload && !params.onLoad)
            this.on(EventType.Load, params.onload);
        if (params.onloaderror && !params.onLoadError)
            this.on(EventType.LoadError, params.onloaderror);
        if (params.onplay && !params.onPlay)
            this.on(EventType.Play, params.onplay);
        if (params.onpause && !params.onPause)
            this.on(EventType.Pause, params.onpause);
        if (params.onstop && !params.onStop)
            this.on(EventType.Stop, params.onstop);
        if (params.onloop && !params.onLoop)
            this.on(EventType.Loop, params.onloop);
        if (params.onstatechange && !params.onStateChange)
            this.on(EventType.StateChange, params.onstatechange);
        /**
         * Asset loading
         */
        if (params.assetLoader)
            this.assetLoader = params.assetLoader;
        // Hook up the task queue
        this.taskQueue = new TaskQueueManager(this.eventManager);
        this.init({
            src: this.src,
            buffer: this.buffer,
            riveFile: this.riveFile,
            autoplay: params.autoplay,
            autoBind: params.autoBind,
            animations: params.animations,
            stateMachines: params.stateMachines,
            artboard: params.artboard,
            useOffscreenRenderer: params.useOffscreenRenderer,
        });
    }
    Object.defineProperty(Rive.prototype, "viewModelCount", {
        get: function () {
            return this.file.viewModelCount();
        },
        enumerable: false,
        configurable: true
    });
    // Alternative constructor to build a Rive instance from an interface/object
    Rive.new = function (params) {
        console.warn("This function is deprecated: please use `new Rive({})` instead");
        return new Rive(params);
    };
    // Event handler for when audio context becomes available
    Rive.prototype.onSystemAudioChanged = function () {
        this.volume = this._volume;
    };
    // Initializes the Rive object either from constructor or load()
    Rive.prototype.init = function (_a) {
        var _this = this;
        var src = _a.src, buffer = _a.buffer, riveFile = _a.riveFile, animations = _a.animations, stateMachines = _a.stateMachines, artboard = _a.artboard, _b = _a.autoplay, autoplay = _b === void 0 ? false : _b, _c = _a.useOffscreenRenderer, useOffscreenRenderer = _c === void 0 ? false : _c, _d = _a.autoBind, autoBind = _d === void 0 ? false : _d;
        if (this.destroyed) {
            return;
        }
        this.src = src;
        this.buffer = buffer;
        this.riveFile = riveFile;
        // If no source file url specified, it's a bust
        if (!this.src && !this.buffer && !this.riveFile) {
            throw new RiveError(Rive.missingErrorMessage);
        }
        // List of animations that should be initialized.
        var startingAnimationNames = mapToStringArray(animations);
        // List of state machines that should be initialized
        var startingStateMachineNames = mapToStringArray(stateMachines);
        // Ensure loaded is marked as false if loading new file
        this.loaded = false;
        this.readyForPlaying = false;
        // Ensure the runtime is loaded
        RuntimeLoader.awaitInstance()
            .then(function (runtime) {
            if (_this.destroyed) {
                return;
            }
            _this.runtime = runtime;
            _this.removeRiveListeners();
            _this.deleteRiveRenderer();
            // Get the canvas where you want to render the animation and create a renderer
            _this.renderer = _this.runtime.makeRenderer(_this.canvas, useOffscreenRenderer);
            // Initial size adjustment based on devicePixelRatio if no width/height are
            // specified explicitly
            if (!(_this.canvas.width || _this.canvas.height)) {
                _this.resizeDrawingSurfaceToCanvas();
            }
            // Load Rive data from a source uri or a data buffer
            _this.initData(artboard, startingAnimationNames, startingStateMachineNames, autoplay, autoBind)
                .then(function (hasInitialized) {
                if (hasInitialized) {
                    return _this.setupRiveListeners();
                }
            })
                .catch(function (e) {
                console.error(e);
            });
        })
            .catch(function (e) {
            console.error(e);
        });
    };
    /**
     * Setup Rive Listeners on the canvas
     * @param riveListenerOptions - Enables TouchEvent events on the canvas. Set to true to allow
     * touch scrolling on the canvas element on touch-enabled devices
     * i.e. { isTouchScrollEnabled: true }
     */
    Rive.prototype.setupRiveListeners = function (riveListenerOptions) {
        var _this = this;
        if (this.eventCleanup) {
            this.eventCleanup();
        }
        if (!this.shouldDisableRiveListeners) {
            var activeStateMachines = (this.animator.stateMachines || [])
                .filter(function (sm) { return sm.playing && _this.runtime.hasListeners(sm.instance); })
                .map(function (sm) { return sm.instance; });
            var touchScrollEnabledOption = this.isTouchScrollEnabled;
            var dispatchPointerExit = this.dispatchPointerExit;
            var enableMultiTouch = this.enableMultiTouch;
            if (riveListenerOptions &&
                "isTouchScrollEnabled" in riveListenerOptions) {
                touchScrollEnabledOption = riveListenerOptions.isTouchScrollEnabled;
            }
            this.eventCleanup = (0,_utils__WEBPACK_IMPORTED_MODULE_3__.registerTouchInteractions)({
                canvas: this.canvas,
                artboard: this.artboard,
                stateMachines: activeStateMachines,
                renderer: this.renderer,
                rive: this.runtime,
                fit: this._layout.runtimeFit(this.runtime),
                alignment: this._layout.runtimeAlignment(this.runtime),
                isTouchScrollEnabled: touchScrollEnabledOption,
                dispatchPointerExit: dispatchPointerExit,
                enableMultiTouch: enableMultiTouch,
                layoutScaleFactor: this._layout.layoutScaleFactor,
            });
        }
    };
    /**
     * Remove Rive Listeners setup on the canvas
     */
    Rive.prototype.removeRiveListeners = function () {
        if (this.eventCleanup) {
            this.eventCleanup();
            this.eventCleanup = null;
        }
    };
    /**
     * If the instance has audio and the system audio is not ready
     * we hook the instance to the audio manager
     */
    Rive.prototype.initializeAudio = function () {
        var _this = this;
        var _a;
        // Initialize audio if needed
        if (audioManager.status == SystemAudioStatus.UNAVAILABLE) {
            if (this.file.hasAudio ||
                (((_a = this.artboard) === null || _a === void 0 ? void 0 : _a.hasAudio) && this._audioEventListener === null)) {
                this._audioEventListener = {
                    type: EventType.AudioStatusChange,
                    callback: function () { return _this.onSystemAudioChanged(); },
                };
                audioManager.add(this._audioEventListener);
                audioManager.establishAudio();
            }
        }
    };
    Rive.prototype.initArtboardSize = function () {
        if (!this.artboard)
            return;
        // Use preset values if they are not undefined
        this._artboardWidth = this.artboard.width =
            this._artboardWidth || this.artboard.width;
        this._artboardHeight = this.artboard.height =
            this._artboardHeight || this.artboard.height;
    };
    // Initializes runtime with Rive data and preps for playing.
    // Returns true for successful initialization.
    Rive.prototype.initData = function (artboardName, animationNames, stateMachineNames, autoplay, autoBind) {
        return __awaiter(this, void 0, void 0, function () {
            var riveFile, error_2, msg;
            var _a;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        _b.trys.push([0, 3, , 4]);
                        if (!(this.riveFile == null)) return [3 /*break*/, 2];
                        riveFile = new RiveFile({
                            src: this.src,
                            buffer: this.buffer,
                            enableRiveAssetCDN: this.enableRiveAssetCDN,
                            assetLoader: this.assetLoader,
                        });
                        this.riveFile = riveFile;
                        return [4 /*yield*/, riveFile.init()];
                    case 1:
                        _b.sent();
                        if (this.destroyed) {
                            // In the very unlikely scenario where the rive file created by this Rive is shared by
                            // another rive file, we only want to destroy it if this file is the only owner.
                            riveFile.destroyIfUnused();
                            return [2 /*return*/, false];
                        }
                        _b.label = 2;
                    case 2:
                        this.file = this.riveFile.getInstance();
                        // Initialize and draw frame
                        this.initArtboard(artboardName, animationNames, stateMachineNames, autoplay, autoBind);
                        // Initialize the artboard size
                        this.initArtboardSize();
                        // Check for audio
                        this.initializeAudio();
                        // Everything's set up, emit a load event
                        this.loaded = true;
                        this.eventManager.fire({
                            type: EventType.Load,
                            data: (_a = this.src) !== null && _a !== void 0 ? _a : "buffer",
                        });
                        // Only initialize paused state machines after the load event has been fired
                        // to allow users to initialize inputs and view models before the first advance
                        this.animator.advanceIfPaused();
                        // Flag ready for playback commands and clear the task queue; this order
                        // is important or it may infinitely recurse
                        this.readyForPlaying = true;
                        this.taskQueue.process();
                        this.drawFrame();
                        return [2 /*return*/, true];
                    case 3:
                        error_2 = _b.sent();
                        msg = resolveErrorMessage(error_2);
                        console.warn(msg);
                        this.eventManager.fire({ type: EventType.LoadError, data: msg });
                        return [2 /*return*/, Promise.reject(msg)];
                    case 4: return [2 /*return*/];
                }
            });
        });
    };
    // Initialize for playback
    Rive.prototype.initArtboard = function (artboardName, animationNames, stateMachineNames, autoplay, autoBind) {
        if (!this.file) {
            return;
        }
        // Fetch the artboard
        var rootArtboard = artboardName
            ? this.file.artboardByName(artboardName)
            : this.file.defaultArtboard();
        // Check we have a working artboard
        if (!rootArtboard) {
            var msg = "Invalid artboard name or no default artboard";
            console.warn(msg);
            this.eventManager.fire({ type: EventType.LoadError, data: msg });
            return;
        }
        this.artboard = rootArtboard;
        rootArtboard.volume = this._volume * audioManager.systemVolume;
        // Initialize the animator
        this.animator = new Animator(this.runtime, this.artboard, this.eventManager);
        // Initialize the animations; as loaded hasn't happened yet, we need to
        // suppress firing the play/pause events until the load event has fired. To
        // do this we tell the animator to suppress firing events, and add event
        // firing to the task queue.
        var instanceNames;
        if (animationNames.length > 0 || stateMachineNames.length > 0) {
            instanceNames = animationNames.concat(stateMachineNames);
            this.animator.initLinearAnimations(animationNames, autoplay);
            this.animator.initStateMachines(stateMachineNames, autoplay);
        }
        else {
            instanceNames = [this.animator.atLeastOne(autoplay, false)];
        }
        // Queue up firing the playback events
        this.taskQueue.add({
            event: {
                type: autoplay ? EventType.Play : EventType.Pause,
                data: instanceNames,
            },
        });
        if (autoBind) {
            var viewModel = this.file.defaultArtboardViewModel(rootArtboard);
            if (viewModel !== null) {
                var runtimeInstance = viewModel.defaultInstance();
                if (runtimeInstance !== null) {
                    var viewModelInstance = new ViewModelInstance(runtimeInstance, null);
                    (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(viewModelInstance, viewModelInstance.runtimeInstance);
                    this.bindViewModelInstance(viewModelInstance);
                }
            }
        }
    };
    // Draws the current artboard frame
    Rive.prototype.drawFrame = function () {
        var _a, _b;
        if ((_a = document === null || document === void 0 ? void 0 : document.timeline) === null || _a === void 0 ? void 0 : _a.currentTime) {
            if (this.loaded && this.artboard && !this.frameRequestId) {
                this._boundDraw(document.timeline.currentTime);
                (_b = this.runtime) === null || _b === void 0 ? void 0 : _b.resolveAnimationFrame();
            }
        }
        else {
            this.scheduleRendering();
        }
    };
    Rive.prototype._canvasSizeChanged = function () {
        var changed = false;
        if (this.canvas) {
            if (this.canvas.width !== this._currentCanvasWidth) {
                this._currentCanvasWidth = this.canvas.width;
                changed = true;
            }
            if (this.canvas.height !== this._currentCanvasHeight) {
                this._currentCanvasHeight = this.canvas.height;
                changed = true;
            }
        }
        return changed;
    };
    /**
     * Draw rendering loop; renders animation frames at the correct time interval.
     * @param time the time at which to render a frame
     */
    Rive.prototype.draw = function (time, onSecond) {
        var _a;
        // Clear the frameRequestId, as we're now rendering a fresh frame
        this.frameRequestId = null;
        var before = performance.now();
        // On the first pass, make sure lastTime has a valid value
        if (!this.lastRenderTime) {
            this.lastRenderTime = time;
        }
        // Handle the onSecond callback
        this.renderSecondTimer += time - this.lastRenderTime;
        if (this.renderSecondTimer > 5000) {
            this.renderSecondTimer = 0;
            onSecond === null || onSecond === void 0 ? void 0 : onSecond();
        }
        // Calculate the elapsed time between frames in seconds
        var elapsedTime = (time - this.lastRenderTime) / 1000;
        this.lastRenderTime = time;
        // - Advance non-paused animations by the elapsed number of seconds
        // - Advance any animations that require scrubbing
        // - Advance to the first frame even when autoplay is false
        var activeAnimations = this.animator.animations
            .filter(function (a) { return a.playing || a.needsScrub; })
            // The scrubbed animations must be applied first to prevent weird artifacts
            // if the playing animations conflict with the scrubbed animating attribuates.
            .sort(function (first) { return (first.needsScrub ? -1 : 1); });
        for (var _i = 0, activeAnimations_1 = activeAnimations; _i < activeAnimations_1.length; _i++) {
            var animation = activeAnimations_1[_i];
            animation.advance(elapsedTime);
            if (animation.instance.didLoop) {
                animation.loopCount += 1;
            }
            animation.apply(1.0);
        }
        // - Advance non-paused state machines by the elapsed number of seconds
        // - Advance to the first frame even when autoplay is false
        var activeStateMachines = this.animator.stateMachines.filter(function (a) { return a.playing; });
        for (var _b = 0, activeStateMachines_1 = activeStateMachines; _b < activeStateMachines_1.length; _b++) {
            var stateMachine = activeStateMachines_1[_b];
            // Check for events before the current frame's state machine advance
            var numEventsReported = stateMachine.reportedEventCount();
            if (numEventsReported) {
                for (var i = 0; i < numEventsReported; i++) {
                    var event_1 = stateMachine.reportedEventAt(i);
                    if (event_1) {
                        if (event_1.type === RiveEventType.OpenUrl) {
                            this.eventManager.fire({
                                type: EventType.RiveEvent,
                                data: event_1,
                            });
                            // Handle the event side effect if explicitly enabled
                            if (this.automaticallyHandleEvents) {
                                var newAnchorTag = document.createElement("a");
                                var _c = event_1, url = _c.url, target = _c.target;
                                var sanitizedUrl = (0,_utils__WEBPACK_IMPORTED_MODULE_3__.sanitizeUrl)(url);
                                url && newAnchorTag.setAttribute("href", sanitizedUrl);
                                target && newAnchorTag.setAttribute("target", target);
                                if (sanitizedUrl && sanitizedUrl !== _utils__WEBPACK_IMPORTED_MODULE_3__.BLANK_URL) {
                                    newAnchorTag.click();
                                }
                            }
                        }
                        else {
                            this.eventManager.fire({
                                type: EventType.RiveEvent,
                                data: event_1,
                            });
                        }
                    }
                }
            }
            stateMachine.advanceAndApply(elapsedTime);
            // stateMachine.instance.apply(this.artboard);
        }
        // Once the animations have been applied to the artboard, advance it
        // by the elapsed time.
        if (this.animator.stateMachines.length == 0) {
            this.artboard.advance(elapsedTime);
        }
        var renderer = this.renderer;
        // Do not draw on 0 canvas size
        if (!this._hasZeroSize) {
            // If there was no dirt on this frame, do not clear and draw
            if (this.drawOptimization == DrawOptimizationOptions.AlwaysDraw ||
                this.artboard.didChange() ||
                this._needsRedraw ||
                this._canvasSizeChanged()) {
                // Canvas must be wiped to prevent artifacts
                renderer.clear();
                renderer.save();
                // Update the renderer alignment if necessary
                this.alignRenderer();
                this.artboard.draw(renderer);
                renderer.restore();
                renderer.flush();
                this._needsRedraw = false;
            }
        }
        // Check for any animations that looped
        this.animator.handleLooping();
        // Check for any state machines that had a state change
        this.animator.handleStateChanges();
        // Report advanced time
        this.animator.handleAdvancing(elapsedTime);
        // Add duration to create frame to durations array
        this.frameCount++;
        var after = performance.now();
        this.frameTimes.push(after);
        this.durations.push(after - before);
        while (this.frameTimes[0] <= after - 1000) {
            this.frameTimes.shift();
            this.durations.shift();
        }
        (_a = this._viewModelInstance) === null || _a === void 0 ? void 0 : _a.handleCallbacks();
        // Calling requestAnimationFrame will rerun draw() at the correct rate:
        // https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Basic_animations
        if (this.animator.isPlaying) {
            // Request a new rendering frame
            this.scheduleRendering();
        }
        else if (this.animator.isPaused) {
            // Reset the end time so on playback it starts at the correct frame
            this.lastRenderTime = 0;
        }
        else if (this.animator.isStopped) {
            // Reset animation instances, artboard and time
            // TODO: implement this properly when we have instancing
            // this.initArtboard();
            // this.drawFrame();
            this.lastRenderTime = 0;
        }
    };
    /**
     * Align the renderer
     */
    Rive.prototype.alignRenderer = function () {
        var _a = this, renderer = _a.renderer, runtime = _a.runtime, _layout = _a._layout, artboard = _a.artboard;
        // Align things up safe in the knowledge we can restore if changed
        renderer.align(_layout.runtimeFit(runtime), _layout.runtimeAlignment(runtime), {
            minX: _layout.minX,
            minY: _layout.minY,
            maxX: _layout.maxX,
            maxY: _layout.maxY,
        }, artboard.bounds, this._devicePixelRatioUsed * _layout.layoutScaleFactor);
    };
    Object.defineProperty(Rive.prototype, "fps", {
        get: function () {
            return this.durations.length;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "frameTime", {
        get: function () {
            if (this.durations.length === 0) {
                return 0;
            }
            return (this.durations.reduce(function (a, b) { return a + b; }, 0) / this.durations.length).toFixed(4);
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Cleans up all Wasm-generated objects that need to be manually destroyed:
     * artboard instances, animation instances, state machine instances,
     * renderer instance, file and runtime.
     *
     * Once this is called, you will need to initialise a new instance of the
     * Rive class
     */
    Rive.prototype.cleanup = function () {
        var _a, _b;
        this.destroyed = true;
        // Stop the renderer if it hasn't already been stopped.
        this.stopRendering();
        // Clean up any artboard, animation or state machine instances.
        this.cleanupInstances();
        // Remove from observer
        if (this._observed !== null) {
            observers.remove(this._observed);
        }
        this.removeRiveListeners();
        if (this.file) {
            (_a = this.riveFile) === null || _a === void 0 ? void 0 : _a.cleanup();
            this.file = null;
        }
        this.riveFile = null;
        this.deleteRiveRenderer();
        if (this._audioEventListener !== null) {
            audioManager.remove(this._audioEventListener);
            this._audioEventListener = null;
        }
        (_b = this._viewModelInstance) === null || _b === void 0 ? void 0 : _b.cleanup();
        this._viewModelInstance = null;
        this._dataEnums = null;
    };
    /**
     * Cleans up the Renderer object. Only call this API if you no longer
     * need to render Rive content in your session.
     */
    Rive.prototype.deleteRiveRenderer = function () {
        var _a;
        (_a = this.renderer) === null || _a === void 0 ? void 0 : _a.delete();
        this.renderer = null;
    };
    /**
     * Cleans up any Wasm-generated objects that need to be manually destroyed:
     * artboard instances, animation instances, state machine instances.
     *
     * Once this is called, things will need to be reinitialized or bad things
     * might happen.
     */
    Rive.prototype.cleanupInstances = function () {
        if (this.eventCleanup !== null) {
            this.eventCleanup();
        }
        // Delete all animation and state machine instances
        this.stop();
        if (this.artboard) {
            this.artboard.delete();
            this.artboard = null;
        }
    };
    /**
     * Tries to query the setup Artboard for a text run node with the given name.
     *
     * @param textRunName - Name of the text run node associated with a text object
     * @returns - TextValueRun node or undefined if the text run cannot be queried
     */
    Rive.prototype.retrieveTextRun = function (textRunName) {
        var _a;
        if (!textRunName) {
            console.warn("No text run name provided");
            return;
        }
        if (!this.artboard) {
            console.warn("Tried to access text run, but the Artboard is null");
            return;
        }
        var textRun = this.artboard.textRun(textRunName);
        if (!textRun) {
            console.warn("Could not access a text run with name '".concat(textRunName, "' in the '").concat((_a = this.artboard) === null || _a === void 0 ? void 0 : _a.name, "' Artboard. Note that you must rename a text run node in the Rive editor to make it queryable at runtime."));
            return;
        }
        return textRun;
    };
    /**
     * Returns a string from a given text run node name, or undefined if the text run
     * cannot be queried.
     *
     * @param textRunName - Name of the text run node associated with a text object
     * @returns - String value of the text run node or undefined
     */
    Rive.prototype.getTextRunValue = function (textRunName) {
        var textRun = this.retrieveTextRun(textRunName);
        return textRun ? textRun.text : undefined;
    };
    /**
     * Sets a text value for a given text run node name if possible
     *
     * @param textRunName - Name of the text run node associated with a text object
     * @param textRunValue - String value to set on the text run node
     */
    Rive.prototype.setTextRunValue = function (textRunName, textRunValue) {
        var textRun = this.retrieveTextRun(textRunName);
        if (textRun) {
            textRun.text = textRunValue;
        }
    };
    // Plays specified animations; if none specified, it unpauses everything.
    Rive.prototype.play = function (animationNames, autoplay) {
        var _this = this;
        animationNames = mapToStringArray(animationNames);
        // If the file's not loaded, queue up the play
        if (!this.readyForPlaying) {
            this.taskQueue.add({
                action: function () { return _this.play(animationNames, autoplay); },
            });
            return;
        }
        this.animator.play(animationNames);
        if (this.eventCleanup) {
            this.eventCleanup();
        }
        this.setupRiveListeners();
        this.startRendering();
    };
    // Pauses specified animations; if none specified, pauses all.
    Rive.prototype.pause = function (animationNames) {
        var _this = this;
        animationNames = mapToStringArray(animationNames);
        // If the file's not loaded, early out, nothing to pause
        if (!this.readyForPlaying) {
            this.taskQueue.add({
                action: function () { return _this.pause(animationNames); },
            });
            return;
        }
        if (this.eventCleanup) {
            this.eventCleanup();
        }
        this.animator.pause(animationNames);
    };
    Rive.prototype.scrub = function (animationNames, value) {
        var _this = this;
        animationNames = mapToStringArray(animationNames);
        // If the file's not loaded, early out, nothing to pause
        if (!this.readyForPlaying) {
            this.taskQueue.add({
                action: function () { return _this.scrub(animationNames, value); },
            });
            return;
        }
        // Scrub the animation time; we draw a single frame here so that if
        // nothing's currently playing, the scrubbed animation is still rendered/
        this.animator.scrub(animationNames, value || 0);
        this.drawFrame();
    };
    // Stops specified animations; if none specifies, stops them all.
    Rive.prototype.stop = function (animationNames) {
        var _this = this;
        animationNames = mapToStringArray(animationNames);
        // If the file's not loaded, early out, nothing to pause
        if (!this.readyForPlaying) {
            this.taskQueue.add({
                action: function () { return _this.stop(animationNames); },
            });
            return;
        }
        // If there is no artboard, this.animator will be undefined
        if (this.animator) {
            this.animator.stop(animationNames);
        }
        if (this.eventCleanup) {
            this.eventCleanup();
        }
    };
    /**
     * Resets the animation
     * @param artboard the name of the artboard, or default if none given
     * @param animations the names of animations for playback
     * @param stateMachines the names of state machines for playback
     * @param autoplay whether to autoplay when reset, defaults to false
     *
     */
    Rive.prototype.reset = function (params) {
        var _a, _b;
        // Get the current artboard, animations, state machines, and playback states
        var artBoardName = params === null || params === void 0 ? void 0 : params.artboard;
        var animationNames = mapToStringArray(params === null || params === void 0 ? void 0 : params.animations);
        var stateMachineNames = mapToStringArray(params === null || params === void 0 ? void 0 : params.stateMachines);
        var autoplay = (_a = params === null || params === void 0 ? void 0 : params.autoplay) !== null && _a !== void 0 ? _a : false;
        var autoBind = (_b = params === null || params === void 0 ? void 0 : params.autoBind) !== null && _b !== void 0 ? _b : false;
        // Stop everything and clean up
        this.cleanupInstances();
        // Reinitialize an artboard instance with the state
        this.initArtboard(artBoardName, animationNames, stateMachineNames, autoplay, autoBind);
        this.taskQueue.process();
    };
    // Loads a new Rive file, keeping listeners in place
    Rive.prototype.load = function (params) {
        this.file = null;
        // Stop all animations
        this.stop();
        // Reinitialize
        this.init(params);
    };
    Object.defineProperty(Rive.prototype, "layout", {
        /**
         * Returns the current layout. Note that layout should be treated as
         * immutable. If you want to change the layout, create a new one use the
         * layout setter
         */
        get: function () {
            return this._layout;
        },
        // Sets a new layout
        set: function (layout) {
            this._layout = layout;
            // If the maxX or maxY are 0, then set them to the canvas width and height
            if (!layout.maxX || !layout.maxY) {
                this.resizeToCanvas();
            }
            if (this.loaded && !this.animator.isPlaying) {
                this.drawFrame();
            }
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Sets the layout bounds to the current canvas size; this is typically called
     * when the canvas is resized
     */
    Rive.prototype.resizeToCanvas = function () {
        this._layout = this.layout.copyWith({
            minX: 0,
            minY: 0,
            maxX: this.canvas.width,
            maxY: this.canvas.height,
        });
    };
    /**
     * Accounts for devicePixelRatio as a multiplier to render the size of the canvas drawing surface.
     * Uses the size of the backing canvas to set new width/height attributes. Need to re-render
     * and resize the layout to match the new drawing surface afterwards.
     * Useful function for consumers to include in a window resize listener.
     *
     * This method will set the {@link devicePixelRatioUsed} property.
     *
     * Optionally, you can provide a {@link customDevicePixelRatio} to provide a
     * custom value.
     */
    Rive.prototype.resizeDrawingSurfaceToCanvas = function (customDevicePixelRatio) {
        if (this.canvas instanceof HTMLCanvasElement && !!window) {
            var _a = this.canvas.getBoundingClientRect(), width = _a.width, height = _a.height;
            var dpr = customDevicePixelRatio || window.devicePixelRatio || 1;
            this.devicePixelRatioUsed = dpr;
            this.canvas.width = dpr * width;
            this.canvas.height = dpr * height;
            this._needsRedraw = true;
            this.resizeToCanvas();
            this.drawFrame();
            if (this.layout.fit === Fit.Layout) {
                var scaleFactor = this._layout.layoutScaleFactor;
                this.artboard.width = width / scaleFactor;
                this.artboard.height = height / scaleFactor;
            }
        }
    };
    Object.defineProperty(Rive.prototype, "source", {
        // Returns the animation source, which may be undefined
        get: function () {
            return this.src;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "activeArtboard", {
        /**
         * Returns the name of the active artboard
         */
        get: function () {
            return this.artboard ? this.artboard.name : "";
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "animationNames", {
        // Returns a list of animation names on the chosen artboard
        get: function () {
            // If the file's not loaded, we got nothing to return
            if (!this.loaded || !this.artboard) {
                return [];
            }
            var animationNames = [];
            for (var i = 0; i < this.artboard.animationCount(); i++) {
                animationNames.push(this.artboard.animationByIndex(i).name);
            }
            return animationNames;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "stateMachineNames", {
        /**
         * Returns a list of state machine names from the current artboard
         */
        get: function () {
            // If the file's not loaded, we got nothing to return
            if (!this.loaded || !this.artboard) {
                return [];
            }
            var stateMachineNames = [];
            for (var i = 0; i < this.artboard.stateMachineCount(); i++) {
                stateMachineNames.push(this.artboard.stateMachineByIndex(i).name);
            }
            return stateMachineNames;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Returns the inputs for the specified instanced state machine, or an empty
     * list if the name is invalid or the state machine is not instanced
     * @param name the state machine name
     * @returns the inputs for the named state machine
     */
    Rive.prototype.stateMachineInputs = function (name) {
        // If the file's not loaded, early out, nothing to pause
        if (!this.loaded) {
            return;
        }
        var stateMachine = this.animator.stateMachines.find(function (m) { return m.name === name; });
        return stateMachine === null || stateMachine === void 0 ? void 0 : stateMachine.inputs;
    };
    // Returns the input with the provided name at the given path
    Rive.prototype.retrieveInputAtPath = function (name, path) {
        if (!name) {
            console.warn("No input name provided for path '".concat(path, "'"));
            return;
        }
        if (!this.artboard) {
            console.warn("Tried to access input: '".concat(name, "', at path: '").concat(path, "', but the Artboard is null"));
            return;
        }
        var input = this.artboard.inputByPath(name, path);
        if (!input) {
            console.warn("Could not access an input with name: '".concat(name, "', at path:'").concat(path, "'"));
            return;
        }
        return input;
    };
    /**
     * Set the boolean input with the provided name at the given path with value
     * @param input the state machine input name
     * @param value the value to set the input to
     * @param path the path the input is located at an artboard level
     */
    Rive.prototype.setBooleanStateAtPath = function (inputName, value, path) {
        var input = this.retrieveInputAtPath(inputName, path);
        if (!input)
            return;
        if (input.type === StateMachineInputType.Boolean) {
            input.asBool().value = value;
        }
        else {
            console.warn("Input with name: '".concat(inputName, "', at path:'").concat(path, "' is not a boolean"));
        }
    };
    /**
     * Set the number input with the provided name at the given path with value
     * @param input the state machine input name
     * @param value the value to set the input to
     * @param path the path the input is located at an artboard level
     */
    Rive.prototype.setNumberStateAtPath = function (inputName, value, path) {
        var input = this.retrieveInputAtPath(inputName, path);
        if (!input)
            return;
        if (input.type === StateMachineInputType.Number) {
            input.asNumber().value = value;
        }
        else {
            console.warn("Input with name: '".concat(inputName, "', at path:'").concat(path, "' is not a number"));
        }
    };
    /**
     * Fire the trigger with the provided name at the given path
     * @param input the state machine input name
     * @param path the path the input is located at an artboard level
     */
    Rive.prototype.fireStateAtPath = function (inputName, path) {
        var input = this.retrieveInputAtPath(inputName, path);
        if (!input)
            return;
        if (input.type === StateMachineInputType.Trigger) {
            input.asTrigger().fire();
        }
        else {
            console.warn("Input with name: '".concat(inputName, "', at path:'").concat(path, "' is not a trigger"));
        }
    };
    // Returns the TextValueRun object for the provided name at the given path
    Rive.prototype.retrieveTextAtPath = function (name, path) {
        if (!name) {
            console.warn("No text name provided for path '".concat(path, "'"));
            return;
        }
        if (!path) {
            console.warn("No path provided for text '".concat(name, "'"));
            return;
        }
        if (!this.artboard) {
            console.warn("Tried to access text: '".concat(name, "', at path: '").concat(path, "', but the Artboard is null"));
            return;
        }
        var text = this.artboard.textByPath(name, path);
        if (!text) {
            console.warn("Could not access text with name: '".concat(name, "', at path:'").concat(path, "'"));
            return;
        }
        return text;
    };
    /**
     * Retrieves the text value for a specified text run at a given path
     * @param textName The name of the text run
     * @param path The path to the text run within the artboard
     * @returns The text value of the text run, or undefined if not found
     *
     * @example
     * // Get the text value for a text run named "title" at one nested artboard deep
     * const titleText = riveInstance.getTextRunValueAtPath("title", "artboard1");
     *
     * @example
     * // Get the text value for a text run named "subtitle" within a nested group two artboards deep
     * const subtitleText = riveInstance.getTextRunValueAtPath("subtitle", "group/nestedGroup");
     *
     * @remarks
     * If the text run cannot be found at the specified path, a warning will be logged to the console.
     */
    Rive.prototype.getTextRunValueAtPath = function (textName, path) {
        var run = this.retrieveTextAtPath(textName, path);
        if (!run) {
            console.warn("Could not get text with name: '".concat(textName, "', at path:'").concat(path, "'"));
            return;
        }
        return run.text;
    };
    /**
     * Sets the text value for a specified text run at a given path
     * @param textName The name of the text run
     * @param value The new text value to set
     * @param path The path to the text run within the artboard
     * @returns void
     *
     * @example
     * // Set the text value for a text run named "title" at one nested artboard deep
     * riveInstance.setTextRunValueAtPath("title", "New Title", "artboard1");
     *
     * @example
     * // Set the text value for a text run named "subtitle" within a nested group two artboards deep
     * riveInstance.setTextRunValueAtPath("subtitle", "New Subtitle", "group/nestedGroup");
     *
     * @remarks
     * If the text run cannot be found at the specified path, a warning will be logged to the console.
     */
    Rive.prototype.setTextRunValueAtPath = function (textName, value, path) {
        var run = this.retrieveTextAtPath(textName, path);
        if (!run) {
            console.warn("Could not set text with name: '".concat(textName, "', at path:'").concat(path, "'"));
            return;
        }
        run.text = value;
    };
    Object.defineProperty(Rive.prototype, "playingStateMachineNames", {
        // Returns a list of playing machine names
        get: function () {
            // If the file's not loaded, we got nothing to return
            if (!this.loaded) {
                return [];
            }
            return this.animator.stateMachines
                .filter(function (m) { return m.playing; })
                .map(function (m) { return m.name; });
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "playingAnimationNames", {
        // Returns a list of playing animation names
        get: function () {
            // If the file's not loaded, we got nothing to return
            if (!this.loaded) {
                return [];
            }
            return this.animator.animations.filter(function (a) { return a.playing; }).map(function (a) { return a.name; });
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "pausedAnimationNames", {
        // Returns a list of paused animation names
        get: function () {
            // If the file's not loaded, we got nothing to return
            if (!this.loaded) {
                return [];
            }
            return this.animator.animations
                .filter(function (a) { return !a.playing; })
                .map(function (a) { return a.name; });
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "pausedStateMachineNames", {
        /**
         *  Returns a list of paused machine names
         * @returns a list of state machine names that are paused
         */
        get: function () {
            // If the file's not loaded, we got nothing to return
            if (!this.loaded) {
                return [];
            }
            return this.animator.stateMachines
                .filter(function (m) { return !m.playing; })
                .map(function (m) { return m.name; });
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "isPlaying", {
        /**
         * @returns true if any animation is playing
         */
        get: function () {
            return this.animator.isPlaying;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "isPaused", {
        /**
         * @returns true if all instanced animations are paused
         */
        get: function () {
            return this.animator.isPaused;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "isStopped", {
        /**
         * @returns true if no animations are playing or paused
         */
        get: function () {
            var _a, _b;
            return (_b = (_a = this.animator) === null || _a === void 0 ? void 0 : _a.isStopped) !== null && _b !== void 0 ? _b : true;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "bounds", {
        /**
         * @returns the bounds of the current artboard, or undefined if the artboard
         * isn't loaded yet.
         */
        get: function () {
            return this.artboard ? this.artboard.bounds : undefined;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Subscribe to Rive-generated events
     * @param type the type of event to subscribe to
     * @param callback callback to fire when the event occurs
     */
    Rive.prototype.on = function (type, callback) {
        this.eventManager.add({
            type: type,
            callback: callback,
        });
    };
    /**
     * Unsubscribes from a Rive-generated event
     * @param type the type of event to unsubscribe from
     * @param callback the callback to unsubscribe
     */
    Rive.prototype.off = function (type, callback) {
        this.eventManager.remove({
            type: type,
            callback: callback,
        });
    };
    /**
     * Unsubscribes from a Rive-generated event
     * @deprecated
     * @param callback the callback to unsubscribe from
     */
    Rive.prototype.unsubscribe = function (type, callback) {
        console.warn("This function is deprecated: please use `off()` instead.");
        this.off(type, callback);
    };
    /**
     * Unsubscribes all Rive listeners from an event type, or everything if no type is
     * given
     * @param type the type of event to unsubscribe from, or all types if
     * undefined
     */
    Rive.prototype.removeAllRiveEventListeners = function (type) {
        this.eventManager.removeAll(type);
    };
    /**
     * Unsubscribes all listeners from an event type, or everything if no type is
     * given
     * @deprecated
     * @param type the type of event to unsubscribe from, or all types if
     * undefined
     */
    Rive.prototype.unsubscribeAll = function (type) {
        console.warn("This function is deprecated: please use `removeAllRiveEventListeners()` instead.");
        this.removeAllRiveEventListeners(type);
    };
    /**
     * Stops the rendering loop; this is different from pausing in that it doesn't
     * change the state of any animation. It stops rendering from occurring. This
     * is designed for situations such as when Rive isn't visible.
     *
     * The only way to start rendering again is to call `startRendering`.
     * Animations that are marked as playing will start from the position that
     * they would have been at if rendering had not been stopped.
     */
    Rive.prototype.stopRendering = function () {
        if (this.loaded && this.frameRequestId) {
            if (this.runtime.cancelAnimationFrame) {
                this.runtime.cancelAnimationFrame(this.frameRequestId);
            }
            else {
                cancelAnimationFrame(this.frameRequestId);
            }
            this.frameRequestId = null;
        }
    };
    /**
     * Starts the rendering loop if it has been previously stopped. If the
     * renderer is already active, then this will have zero effect.
     */
    Rive.prototype.startRendering = function () {
        this.drawFrame();
    };
    Rive.prototype.scheduleRendering = function () {
        if (this.loaded && this.artboard && !this.frameRequestId) {
            if (this.runtime.requestAnimationFrame) {
                this.frameRequestId = this.runtime.requestAnimationFrame(this._boundDraw);
            }
            else {
                this.frameRequestId = requestAnimationFrame(this._boundDraw);
            }
        }
    };
    /**
     * Enables frames-per-second (FPS) reporting for the runtime
     * If no callback is provided, Rive will append a fixed-position div at the top-right corner of
     * the page with the FPS reading
     * @param fpsCallback - Callback from the runtime during the RAF loop that supplies the FPS value
     */
    Rive.prototype.enableFPSCounter = function (fpsCallback) {
        this.runtime.enableFPSCounter(fpsCallback);
    };
    /**
     * Disables frames-per-second (FPS) reporting for the runtime
     */
    Rive.prototype.disableFPSCounter = function () {
        this.runtime.disableFPSCounter();
    };
    Object.defineProperty(Rive.prototype, "contents", {
        /**
         * Returns the contents of a Rive file: the artboards, animations, and state machines
         */
        get: function () {
            if (!this.loaded) {
                return undefined;
            }
            var riveContents = {
                artboards: [],
            };
            for (var i = 0; i < this.file.artboardCount(); i++) {
                var artboard = this.file.artboardByIndex(i);
                var artboardContents = {
                    name: artboard.name,
                    animations: [],
                    stateMachines: [],
                };
                for (var j = 0; j < artboard.animationCount(); j++) {
                    var animation = artboard.animationByIndex(j);
                    artboardContents.animations.push(animation.name);
                }
                for (var k = 0; k < artboard.stateMachineCount(); k++) {
                    var stateMachine = artboard.stateMachineByIndex(k);
                    var name_1 = stateMachine.name;
                    var instance = new this.runtime.StateMachineInstance(stateMachine, artboard);
                    var inputContents = [];
                    for (var l = 0; l < instance.inputCount(); l++) {
                        var input = instance.input(l);
                        inputContents.push({ name: input.name, type: input.type });
                    }
                    artboardContents.stateMachines.push({
                        name: name_1,
                        inputs: inputContents,
                    });
                }
                riveContents.artboards.push(artboardContents);
            }
            return riveContents;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "volume", {
        /**
         * Getter / Setter for the volume of the artboard
         */
        get: function () {
            if (this.artboard && this.artboard.volume !== this._volume) {
                this._volume = this.artboard.volume;
            }
            return this._volume;
        },
        set: function (value) {
            this._volume = value;
            if (this.artboard) {
                this.artboard.volume = value * audioManager.systemVolume;
            }
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "artboardWidth", {
        /**
         * The width of the artboard.
         *
         * This will return 0 if the artboard is not loaded yet and a custom
         * width has not been set.
         *
         * Do not set this value manually when using {@link resizeDrawingSurfaceToCanvas}
         * with a {@link Layout.fit} of {@link Fit.Layout}, as the artboard width is
         * automatically set.
         */
        get: function () {
            var _a;
            if (this.artboard) {
                return this.artboard.width;
            }
            return (_a = this._artboardWidth) !== null && _a !== void 0 ? _a : 0;
        },
        set: function (value) {
            this._artboardWidth = value;
            if (this.artboard) {
                this.artboard.width = value;
            }
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(Rive.prototype, "artboardHeight", {
        /**
         * The height of the artboard.
         *
         * This will return 0 if the artboard is not loaded yet and a custom
         * height has not been set.
         *
         * Do not set this value manually when using {@link resizeDrawingSurfaceToCanvas}
         * with a {@link Layout.fit} of {@link Fit.Layout}, as the artboard height is
         * automatically set.
         */
        get: function () {
            var _a;
            if (this.artboard) {
                return this.artboard.height;
            }
            return (_a = this._artboardHeight) !== null && _a !== void 0 ? _a : 0;
        },
        set: function (value) {
            this._artboardHeight = value;
            if (this.artboard) {
                this.artboard.height = value;
            }
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Reset the artboard size to its original values.
     */
    Rive.prototype.resetArtboardSize = function () {
        if (this.artboard) {
            this.artboard.resetArtboardSize();
            this._artboardWidth = this.artboard.width;
            this._artboardHeight = this.artboard.height;
        }
        else {
            // If the artboard isn't loaded, we need to reset the custom width and height
            this._artboardWidth = undefined;
            this._artboardHeight = undefined;
        }
    };
    Object.defineProperty(Rive.prototype, "devicePixelRatioUsed", {
        /**
         * The device pixel ratio used in rendering and canvas/artboard resizing.
         *
         * This value will be overidden by the device pixel ratio used in
         * {@link resizeDrawingSurfaceToCanvas}. If you use that method, do not set this value.
         */
        get: function () {
            return this._devicePixelRatioUsed;
        },
        set: function (value) {
            this._devicePixelRatioUsed = value;
        },
        enumerable: false,
        configurable: true
    });
    /**
     * Initialize the data context with the view model instance.
     */
    Rive.prototype.bindViewModelInstance = function (viewModelInstance) {
        var _a;
        if (this.artboard && !this.destroyed) {
            if (viewModelInstance && viewModelInstance.runtimeInstance) {
                viewModelInstance.internalIncrementReferenceCount();
                (_a = this._viewModelInstance) === null || _a === void 0 ? void 0 : _a.cleanup();
                this._viewModelInstance = viewModelInstance;
                if (this.animator.stateMachines.length > 0) {
                    this.animator.stateMachines.forEach(function (stateMachine) {
                        return stateMachine.bindViewModelInstance(viewModelInstance);
                    });
                }
                else {
                    this.artboard.bindViewModelInstance(viewModelInstance.runtimeInstance);
                }
            }
        }
    };
    Object.defineProperty(Rive.prototype, "viewModelInstance", {
        get: function () {
            return this._viewModelInstance;
        },
        enumerable: false,
        configurable: true
    });
    Rive.prototype.viewModelByIndex = function (index) {
        var viewModel = this.file.viewModelByIndex(index);
        if (viewModel !== null) {
            return new ViewModel(viewModel);
        }
        return null;
    };
    Rive.prototype.viewModelByName = function (name) {
        var _a;
        return (_a = this.riveFile) === null || _a === void 0 ? void 0 : _a.viewModelByName(name);
    };
    Rive.prototype.enums = function () {
        if (this._dataEnums === null) {
            var dataEnums = this.file.enums();
            this._dataEnums = dataEnums.map(function (dataEnum) {
                return new DataEnum(dataEnum);
            });
        }
        return this._dataEnums;
    };
    Rive.prototype.defaultViewModel = function () {
        if (this.artboard) {
            var viewModel = this.file.defaultArtboardViewModel(this.artboard);
            if (viewModel) {
                return new ViewModel(viewModel);
            }
        }
        return null;
    };
    /**
     * @deprecated This function is deprecated. For better stability and memory management
     * use `getBindableArtboard()` instead.
     * @param {string} name - The name of the artboard.
     * @returns {Artboard} The artboard to bind to.
     */
    Rive.prototype.getArtboard = function (name) {
        var _a, _b;
        return (_b = (_a = this.riveFile) === null || _a === void 0 ? void 0 : _a.getArtboard(name)) !== null && _b !== void 0 ? _b : null;
    };
    Rive.prototype.getBindableArtboard = function (name) {
        var _a, _b;
        return (_b = (_a = this.riveFile) === null || _a === void 0 ? void 0 : _a.getBindableArtboard(name)) !== null && _b !== void 0 ? _b : null;
    };
    Rive.prototype.getDefaultBindableArtboard = function () {
        var _a, _b;
        return (_b = (_a = this.riveFile) === null || _a === void 0 ? void 0 : _a.getDefaultBindableArtboard()) !== null && _b !== void 0 ? _b : null;
    };
    // Error message for missing source or buffer
    Rive.missingErrorMessage = "Rive source file or data buffer required";
    // Error message for removed rive file
    Rive.cleanupErrorMessage = "Attempt to use file after calling cleanup.";
    return Rive;
}());

var ViewModel = /** @class */ (function () {
    function ViewModel(viewModel) {
        this._viewModel = viewModel;
    }
    Object.defineProperty(ViewModel.prototype, "instanceCount", {
        get: function () {
            return this._viewModel.instanceCount;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ViewModel.prototype, "name", {
        get: function () {
            return this._viewModel.name;
        },
        enumerable: false,
        configurable: true
    });
    ViewModel.prototype.instanceByIndex = function (index) {
        var instance = this._viewModel.instanceByIndex(index);
        if (instance !== null) {
            var viewModelInstance = new ViewModelInstance(instance, null);
            (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(viewModelInstance, instance);
            return viewModelInstance;
        }
        return null;
    };
    ViewModel.prototype.instanceByName = function (name) {
        var instance = this._viewModel.instanceByName(name);
        if (instance !== null) {
            var viewModelInstance = new ViewModelInstance(instance, null);
            (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(viewModelInstance, instance);
            return viewModelInstance;
        }
        return null;
    };
    ViewModel.prototype.defaultInstance = function () {
        var runtimeInstance = this._viewModel.defaultInstance();
        if (runtimeInstance !== null) {
            var viewModelInstance = new ViewModelInstance(runtimeInstance, null);
            (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(viewModelInstance, runtimeInstance);
            return viewModelInstance;
        }
        return null;
    };
    ViewModel.prototype.instance = function () {
        var runtimeInstance = this._viewModel.instance();
        if (runtimeInstance !== null) {
            var viewModelInstance = new ViewModelInstance(runtimeInstance, null);
            (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(viewModelInstance, runtimeInstance);
            return viewModelInstance;
        }
        return null;
    };
    Object.defineProperty(ViewModel.prototype, "properties", {
        get: function () {
            return this._viewModel.getProperties();
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ViewModel.prototype, "instanceNames", {
        get: function () {
            return this._viewModel.getInstanceNames();
        },
        enumerable: false,
        configurable: true
    });
    return ViewModel;
}());

var DataEnum = /** @class */ (function () {
    function DataEnum(dataEnum) {
        this._dataEnum = dataEnum;
    }
    Object.defineProperty(DataEnum.prototype, "name", {
        get: function () {
            return this._dataEnum.name;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(DataEnum.prototype, "values", {
        get: function () {
            return this._dataEnum.values;
        },
        enumerable: false,
        configurable: true
    });
    return DataEnum;
}());

var PropertyType;
(function (PropertyType) {
    PropertyType["Number"] = "number";
    PropertyType["String"] = "string";
    PropertyType["Boolean"] = "boolean";
    PropertyType["Color"] = "color";
    PropertyType["Trigger"] = "trigger";
    PropertyType["Enum"] = "enum";
    PropertyType["List"] = "list";
    PropertyType["Image"] = "image";
    PropertyType["Artboard"] = "artboard";
})(PropertyType || (PropertyType = {}));
var ViewModelInstance = /** @class */ (function () {
    function ViewModelInstance(runtimeInstance, parent) {
        this._parents = [];
        this._children = [];
        this._viewModelInstances = new Map();
        this._propertiesWithCallbacks = [];
        this._referenceCount = 0;
        this.selfUnref = false;
        this._runtimeInstance = runtimeInstance;
        if (parent !== null) {
            this._parents.push(parent);
        }
    }
    Object.defineProperty(ViewModelInstance.prototype, "runtimeInstance", {
        get: function () {
            return this._runtimeInstance;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ViewModelInstance.prototype, "nativeInstance", {
        get: function () {
            return this._runtimeInstance;
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstance.prototype.handleCallbacks = function () {
        if (this._propertiesWithCallbacks.length !== 0) {
            this._propertiesWithCallbacks.forEach(function (property) {
                property.handleCallbacks();
            });
            this._propertiesWithCallbacks.forEach(function (property) {
                property.clearChanges();
            });
        }
        this._children.forEach(function (child) { return child.handleCallbacks(); });
    };
    ViewModelInstance.prototype.addParent = function (parent) {
        if (!this._parents.includes(parent)) {
            this._parents.push(parent);
            if (this._propertiesWithCallbacks.length > 0 ||
                this._children.length > 0) {
                parent.addToViewModelCallbacks(this);
            }
        }
    };
    ViewModelInstance.prototype.removeParent = function (parent) {
        var index = this._parents.indexOf(parent);
        if (index !== -1) {
            var parent_1 = this._parents[index];
            parent_1.removeFromViewModelCallbacks(this);
            this._parents.splice(index, 1);
        }
    };
    /*
     * method for internal use, it shouldn't be called externally
     */
    ViewModelInstance.prototype.addToPropertyCallbacks = function (property) {
        var _this = this;
        if (!this._propertiesWithCallbacks.includes(property)) {
            this._propertiesWithCallbacks.push(property);
            if (this._propertiesWithCallbacks.length > 0) {
                this._parents.forEach(function (parent) {
                    parent.addToViewModelCallbacks(_this);
                });
            }
        }
    };
    /*
     * method for internal use, it shouldn't be called externally
     */
    ViewModelInstance.prototype.removeFromPropertyCallbacks = function (property) {
        var _this = this;
        if (this._propertiesWithCallbacks.includes(property)) {
            this._propertiesWithCallbacks = this._propertiesWithCallbacks.filter(function (prop) { return prop !== property; });
            if (this._children.length === 0 &&
                this._propertiesWithCallbacks.length === 0) {
                this._parents.forEach(function (parent) {
                    parent.removeFromViewModelCallbacks(_this);
                });
            }
        }
    };
    /*
     * method for internal use, it shouldn't be called externally
     */
    ViewModelInstance.prototype.addToViewModelCallbacks = function (instance) {
        var _this = this;
        if (!this._children.includes(instance)) {
            this._children.push(instance);
            this._parents.forEach(function (parent) {
                parent.addToViewModelCallbacks(_this);
            });
        }
    };
    /*
     * method for internal use, it shouldn't be called externally
     */
    ViewModelInstance.prototype.removeFromViewModelCallbacks = function (instance) {
        var _this = this;
        if (this._children.includes(instance)) {
            this._children = this._children.filter(function (child) { return child !== instance; });
            if (this._children.length === 0 &&
                this._propertiesWithCallbacks.length === 0) {
                this._parents.forEach(function (parent) {
                    parent.removeFromViewModelCallbacks(_this);
                });
            }
        }
    };
    ViewModelInstance.prototype.clearCallbacks = function () {
        this._propertiesWithCallbacks.forEach(function (property) {
            property.clearCallbacks();
        });
    };
    ViewModelInstance.prototype.propertyFromPath = function (path, type) {
        var pathSegments = path.split("/");
        return this.propertyFromPathSegments(pathSegments, 0, type);
    };
    ViewModelInstance.prototype.viewModelFromPathSegments = function (pathSegments, index) {
        var viewModelInstance = this.internalViewModelInstance(pathSegments[index]);
        if (viewModelInstance !== null) {
            if (index == pathSegments.length - 1) {
                return viewModelInstance;
            }
            else {
                return viewModelInstance.viewModelFromPathSegments(pathSegments, index++);
            }
        }
        return null;
    };
    ViewModelInstance.prototype.propertyFromPathSegments = function (pathSegments, index, type) {
        var _a, _b, _c, _d, _e, _f, _g, _h, _j, _k, _l, _m, _o, _p, _q, _r, _s, _t;
        if (index < pathSegments.length - 1) {
            var viewModelInstance = this.internalViewModelInstance(pathSegments[index]);
            if (viewModelInstance !== null) {
                return viewModelInstance.propertyFromPathSegments(pathSegments, index + 1, type);
            }
            else {
                return null;
            }
        }
        var instance = null;
        switch (type) {
            case PropertyType.Number:
                instance = (_b = (_a = this._runtimeInstance) === null || _a === void 0 ? void 0 : _a.number(pathSegments[index])) !== null && _b !== void 0 ? _b : null;
                if (instance !== null) {
                    return new ViewModelInstanceNumber(instance, this);
                }
                break;
            case PropertyType.String:
                instance = (_d = (_c = this._runtimeInstance) === null || _c === void 0 ? void 0 : _c.string(pathSegments[index])) !== null && _d !== void 0 ? _d : null;
                if (instance !== null) {
                    return new ViewModelInstanceString(instance, this);
                }
                break;
            case PropertyType.Boolean:
                instance = (_f = (_e = this._runtimeInstance) === null || _e === void 0 ? void 0 : _e.boolean(pathSegments[index])) !== null && _f !== void 0 ? _f : null;
                if (instance !== null) {
                    return new ViewModelInstanceBoolean(instance, this);
                }
                break;
            case PropertyType.Color:
                instance = (_h = (_g = this._runtimeInstance) === null || _g === void 0 ? void 0 : _g.color(pathSegments[index])) !== null && _h !== void 0 ? _h : null;
                if (instance !== null) {
                    return new ViewModelInstanceColor(instance, this);
                }
                break;
            case PropertyType.Trigger:
                instance = (_k = (_j = this._runtimeInstance) === null || _j === void 0 ? void 0 : _j.trigger(pathSegments[index])) !== null && _k !== void 0 ? _k : null;
                if (instance !== null) {
                    return new ViewModelInstanceTrigger(instance, this);
                }
                break;
            case PropertyType.Enum:
                instance = (_m = (_l = this._runtimeInstance) === null || _l === void 0 ? void 0 : _l.enum(pathSegments[index])) !== null && _m !== void 0 ? _m : null;
                if (instance !== null) {
                    return new ViewModelInstanceEnum(instance, this);
                }
                break;
            case PropertyType.List:
                instance = (_p = (_o = this._runtimeInstance) === null || _o === void 0 ? void 0 : _o.list(pathSegments[index])) !== null && _p !== void 0 ? _p : null;
                if (instance !== null) {
                    return new ViewModelInstanceList(instance, this);
                }
                break;
            case PropertyType.Image:
                instance = (_r = (_q = this._runtimeInstance) === null || _q === void 0 ? void 0 : _q.image(pathSegments[index])) !== null && _r !== void 0 ? _r : null;
                if (instance !== null) {
                    return new ViewModelInstanceAssetImage(instance, this);
                }
                break;
            case PropertyType.Artboard:
                instance = (_t = (_s = this._runtimeInstance) === null || _s === void 0 ? void 0 : _s.artboard(pathSegments[index])) !== null && _t !== void 0 ? _t : null;
                if (instance !== null) {
                    return new ViewModelInstanceArtboard(instance, this);
                }
                break;
        }
        return null;
    };
    ViewModelInstance.prototype.internalViewModelInstance = function (name) {
        var _a;
        if (this._viewModelInstances.has(name)) {
            return this._viewModelInstances.get(name);
        }
        var viewModelRuntimeInstance = (_a = this._runtimeInstance) === null || _a === void 0 ? void 0 : _a.viewModel(name);
        if (viewModelRuntimeInstance !== null) {
            var viewModelInstance = new ViewModelInstance(viewModelRuntimeInstance, this);
            (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(viewModelInstance, viewModelRuntimeInstance);
            viewModelInstance.internalIncrementReferenceCount();
            this._viewModelInstances.set(name, viewModelInstance);
            return viewModelInstance;
        }
        return null;
    };
    /**
     * method to access a property instance of type number belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the number property
     */
    ViewModelInstance.prototype.number = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.Number);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a property instance of type string belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the string property
     */
    ViewModelInstance.prototype.string = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.String);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a property instance of type boolean belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the boolean property
     */
    ViewModelInstance.prototype.boolean = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.Boolean);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a property instance of type color belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the ttrigger property
     */
    ViewModelInstance.prototype.color = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.Color);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a property instance of type trigger belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the trigger property
     */
    ViewModelInstance.prototype.trigger = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.Trigger);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a property instance of type enum belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the enum property
     */
    ViewModelInstance.prototype.enum = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.Enum);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a property instance of type list belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the list property
     */
    ViewModelInstance.prototype.list = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.List);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a view model property instance belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the image property
     */
    ViewModelInstance.prototype.image = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.Image);
        return viewmodelInstanceValue;
    };
    /**
     * method to access an artboard property instance belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the image property
     */
    ViewModelInstance.prototype.artboard = function (path) {
        var viewmodelInstanceValue = this.propertyFromPath(path, PropertyType.Artboard);
        return viewmodelInstanceValue;
    };
    /**
     * method to access a view model property instance belonging
     * to the view model instance or to a nested view model instance
     * @param path - path to the view model property
     */
    ViewModelInstance.prototype.viewModel = function (path) {
        var pathSegments = path.split("/");
        var parentViewModelInstance = pathSegments.length > 1
            ? this.viewModelFromPathSegments(pathSegments.slice(0, pathSegments.length - 1), 0)
            : this;
        if (parentViewModelInstance != null) {
            return parentViewModelInstance.internalViewModelInstance(pathSegments[pathSegments.length - 1]);
        }
        return null;
    };
    ViewModelInstance.prototype.internalReplaceViewModel = function (name, value) {
        var _a;
        if (value.runtimeInstance !== null) {
            var result = ((_a = this._runtimeInstance) === null || _a === void 0 ? void 0 : _a.replaceViewModel(name, value.runtimeInstance)) ||
                false;
            if (result) {
                value.internalIncrementReferenceCount();
                var oldInstance_1 = this.internalViewModelInstance(name);
                if (oldInstance_1 !== null) {
                    oldInstance_1.removeParent(this);
                    if (this._children.includes(oldInstance_1)) {
                        this._children = this._children.filter(function (child) { return child !== oldInstance_1; });
                    }
                    oldInstance_1.cleanup();
                }
                this._viewModelInstances.set(name, value);
                value.addParent(this);
            }
            return result;
        }
        return false;
    };
    /**
     * method to replace a view model property with another view model value
     * @param path - path to the view model property
     * @param value - view model that will replace the original
     */
    ViewModelInstance.prototype.replaceViewModel = function (path, value) {
        var _a;
        var pathSegments = path.split("/");
        var viewModelInstance = pathSegments.length > 1
            ? this.viewModelFromPathSegments(pathSegments.slice(0, pathSegments.length - 1), 0)
            : this;
        return ((_a = viewModelInstance === null || viewModelInstance === void 0 ? void 0 : viewModelInstance.internalReplaceViewModel(pathSegments[pathSegments.length - 1], value)) !== null && _a !== void 0 ? _a : false);
    };
    /*
     * method to add one to the reference counter of the instance.
     * Use if the file owning the reference is destroyed but the instance needs to stay around
     */
    ViewModelInstance.prototype.incrementReferenceCount = function () {
        var _a;
        this._referenceCount++;
        (_a = this._runtimeInstance) === null || _a === void 0 ? void 0 : _a.incrementReferenceCount();
    };
    /*
     * method to subtract one to the reference counter of the instance.
     * Use if incrementReferenceCount has been called
     */
    ViewModelInstance.prototype.decrementReferenceCount = function () {
        var _a;
        this._referenceCount--;
        (_a = this._runtimeInstance) === null || _a === void 0 ? void 0 : _a.decrementReferenceCount();
    };
    Object.defineProperty(ViewModelInstance.prototype, "properties", {
        get: function () {
            var _a;
            return (((_a = this._runtimeInstance) === null || _a === void 0 ? void 0 : _a.getProperties().map(function (prop) { return (__assign({}, prop)); })) || []);
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstance.prototype.internalIncrementReferenceCount = function () {
        this._referenceCount++;
    };
    ViewModelInstance.prototype.cleanup = function () {
        var _this = this;
        var _a;
        this._referenceCount--;
        if (this._referenceCount <= 0) {
            if (this.selfUnref) {
                (_a = this._runtimeInstance) === null || _a === void 0 ? void 0 : _a.unref();
            }
            this._runtimeInstance = null;
            this.clearCallbacks();
            this._propertiesWithCallbacks = [];
            this._viewModelInstances.forEach(function (value) {
                value.cleanup();
            });
            this._viewModelInstances.clear();
            var children = __spreadArray([], this._children, true);
            this._children.length = 0;
            var parents = __spreadArray([], this._parents, true);
            this._parents.length = 0;
            children.forEach(function (child) {
                child.removeParent(_this);
            });
            parents.forEach(function (parent) {
                parent.removeFromViewModelCallbacks(_this);
            });
        }
    };
    return ViewModelInstance;
}());

var ViewModelInstanceValue = /** @class */ (function () {
    function ViewModelInstanceValue(instance, parent) {
        this.callbacks = [];
        this._viewModelInstanceValue = instance;
        this._parentViewModel = parent;
    }
    ViewModelInstanceValue.prototype.on = function (callback) {
        // Since we don't clean the changed flag for properties that don't have listeners,
        // we clean it the first time we add a listener to it
        if (this.callbacks.length === 0) {
            this._viewModelInstanceValue.clearChanges();
        }
        if (!this.callbacks.includes(callback)) {
            this.callbacks.push(callback);
            this._parentViewModel.addToPropertyCallbacks(this);
        }
    };
    ViewModelInstanceValue.prototype.off = function (callback) {
        if (!callback) {
            this.callbacks.length = 0;
        }
        else {
            this.callbacks = this.callbacks.filter(function (cb) { return cb !== callback; });
        }
        if (this.callbacks.length === 0) {
            this._parentViewModel.removeFromPropertyCallbacks(this);
        }
    };
    ViewModelInstanceValue.prototype.internalHandleCallback = function (callback) { };
    ViewModelInstanceValue.prototype.handleCallbacks = function () {
        var _this = this;
        if (this._viewModelInstanceValue.hasChanged) {
            this.callbacks.forEach(function (callback) {
                _this.internalHandleCallback(callback);
            });
        }
    };
    ViewModelInstanceValue.prototype.clearChanges = function () {
        this._viewModelInstanceValue.clearChanges();
    };
    ViewModelInstanceValue.prototype.clearCallbacks = function () {
        this.callbacks.length = 0;
    };
    Object.defineProperty(ViewModelInstanceValue.prototype, "name", {
        get: function () {
            return this._viewModelInstanceValue.name;
        },
        enumerable: false,
        configurable: true
    });
    return ViewModelInstanceValue;
}());

var ViewModelInstanceString = /** @class */ (function (_super) {
    __extends(ViewModelInstanceString, _super);
    function ViewModelInstanceString(instance, parent) {
        return _super.call(this, instance, parent) || this;
    }
    Object.defineProperty(ViewModelInstanceString.prototype, "value", {
        get: function () {
            return this._viewModelInstanceValue.value;
        },
        set: function (val) {
            this._viewModelInstanceValue.value = val;
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceString.prototype.internalHandleCallback = function (callback) {
        callback(this.value);
    };
    return ViewModelInstanceString;
}(ViewModelInstanceValue));

var ViewModelInstanceNumber = /** @class */ (function (_super) {
    __extends(ViewModelInstanceNumber, _super);
    function ViewModelInstanceNumber(instance, parent) {
        return _super.call(this, instance, parent) || this;
    }
    Object.defineProperty(ViewModelInstanceNumber.prototype, "value", {
        get: function () {
            return this._viewModelInstanceValue.value;
        },
        set: function (val) {
            this._viewModelInstanceValue.value = val;
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceNumber.prototype.internalHandleCallback = function (callback) {
        callback(this.value);
    };
    return ViewModelInstanceNumber;
}(ViewModelInstanceValue));

var ViewModelInstanceBoolean = /** @class */ (function (_super) {
    __extends(ViewModelInstanceBoolean, _super);
    function ViewModelInstanceBoolean(instance, parent) {
        return _super.call(this, instance, parent) || this;
    }
    Object.defineProperty(ViewModelInstanceBoolean.prototype, "value", {
        get: function () {
            return this._viewModelInstanceValue.value;
        },
        set: function (val) {
            this._viewModelInstanceValue.value = val;
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceBoolean.prototype.internalHandleCallback = function (callback) {
        callback(this.value);
    };
    return ViewModelInstanceBoolean;
}(ViewModelInstanceValue));

var ViewModelInstanceTrigger = /** @class */ (function (_super) {
    __extends(ViewModelInstanceTrigger, _super);
    function ViewModelInstanceTrigger(instance, parent) {
        return _super.call(this, instance, parent) || this;
    }
    ViewModelInstanceTrigger.prototype.trigger = function () {
        return this._viewModelInstanceValue.trigger();
    };
    ViewModelInstanceTrigger.prototype.internalHandleCallback = function (callback) {
        callback();
    };
    return ViewModelInstanceTrigger;
}(ViewModelInstanceValue));

var ViewModelInstanceEnum = /** @class */ (function (_super) {
    __extends(ViewModelInstanceEnum, _super);
    function ViewModelInstanceEnum(instance, parent) {
        return _super.call(this, instance, parent) || this;
    }
    Object.defineProperty(ViewModelInstanceEnum.prototype, "value", {
        get: function () {
            return this._viewModelInstanceValue.value;
        },
        set: function (val) {
            this._viewModelInstanceValue.value = val;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ViewModelInstanceEnum.prototype, "valueIndex", {
        get: function () {
            return this._viewModelInstanceValue
                .valueIndex;
        },
        set: function (val) {
            this._viewModelInstanceValue.valueIndex = val;
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(ViewModelInstanceEnum.prototype, "values", {
        get: function () {
            return this._viewModelInstanceValue.values;
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceEnum.prototype.internalHandleCallback = function (callback) {
        callback(this.value);
    };
    return ViewModelInstanceEnum;
}(ViewModelInstanceValue));

var ViewModelInstanceList = /** @class */ (function (_super) {
    __extends(ViewModelInstanceList, _super);
    function ViewModelInstanceList(instance, parent) {
        return _super.call(this, instance, parent) || this;
    }
    Object.defineProperty(ViewModelInstanceList.prototype, "length", {
        get: function () {
            return this._viewModelInstanceValue.size;
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceList.prototype.addInstance = function (instance) {
        if (instance.runtimeInstance != null) {
            this._viewModelInstanceValue.addInstance(instance.runtimeInstance);
            instance.addParent(this._parentViewModel);
        }
    };
    ViewModelInstanceList.prototype.addInstanceAt = function (instance, index) {
        if (instance.runtimeInstance != null) {
            if (this._viewModelInstanceValue.addInstanceAt(instance.runtimeInstance, index)) {
                instance.addParent(this._parentViewModel);
                return true;
            }
        }
        return false;
    };
    ViewModelInstanceList.prototype.removeInstance = function (instance) {
        if (instance.runtimeInstance != null) {
            this._viewModelInstanceValue.removeInstance(instance.runtimeInstance);
            instance.removeParent(this._parentViewModel);
        }
    };
    ViewModelInstanceList.prototype.removeInstanceAt = function (index) {
        this._viewModelInstanceValue.removeInstanceAt(index);
    };
    ViewModelInstanceList.prototype.instanceAt = function (index) {
        var runtimeInstance = this._viewModelInstanceValue.instanceAt(index);
        if (runtimeInstance != null) {
            var viewModelInstance = new ViewModelInstance(runtimeInstance, this._parentViewModel);
            (0,_utils__WEBPACK_IMPORTED_MODULE_3__.createFinalization)(viewModelInstance, runtimeInstance);
            return viewModelInstance;
        }
        return null;
    };
    ViewModelInstanceList.prototype.swap = function (a, b) {
        this._viewModelInstanceValue.swap(a, b);
    };
    ViewModelInstanceList.prototype.internalHandleCallback = function (callback) {
        callback();
    };
    return ViewModelInstanceList;
}(ViewModelInstanceValue));

var ViewModelInstanceColor = /** @class */ (function (_super) {
    __extends(ViewModelInstanceColor, _super);
    function ViewModelInstanceColor(instance, parent) {
        return _super.call(this, instance, parent) || this;
    }
    Object.defineProperty(ViewModelInstanceColor.prototype, "value", {
        get: function () {
            return this._viewModelInstanceValue.value;
        },
        set: function (val) {
            this._viewModelInstanceValue.value = val;
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceColor.prototype.rgb = function (r, g, b) {
        this._viewModelInstanceValue.rgb(r, g, b);
    };
    ViewModelInstanceColor.prototype.rgba = function (r, g, b, a) {
        this._viewModelInstanceValue.argb(a, r, g, b);
    };
    ViewModelInstanceColor.prototype.argb = function (a, r, g, b) {
        this._viewModelInstanceValue.argb(a, r, g, b);
    };
    // Value 0 to 255
    ViewModelInstanceColor.prototype.alpha = function (a) {
        this._viewModelInstanceValue.alpha(a);
    };
    // Value 0 to 1
    ViewModelInstanceColor.prototype.opacity = function (o) {
        this._viewModelInstanceValue.alpha(Math.round(Math.max(0, Math.min(1, o)) * 255));
    };
    ViewModelInstanceColor.prototype.internalHandleCallback = function (callback) {
        callback(this.value);
    };
    return ViewModelInstanceColor;
}(ViewModelInstanceValue));

var ViewModelInstanceAssetImage = /** @class */ (function (_super) {
    __extends(ViewModelInstanceAssetImage, _super);
    function ViewModelInstanceAssetImage(instance, root) {
        return _super.call(this, instance, root) || this;
    }
    Object.defineProperty(ViewModelInstanceAssetImage.prototype, "value", {
        set: function (image) {
            var _a;
            this._viewModelInstanceValue.value((_a = image === null || image === void 0 ? void 0 : image.nativeImage) !== null && _a !== void 0 ? _a : null);
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceAssetImage.prototype.internalHandleCallback = function (callback) {
        callback();
    };
    return ViewModelInstanceAssetImage;
}(ViewModelInstanceValue));

var ViewModelInstanceArtboard = /** @class */ (function (_super) {
    __extends(ViewModelInstanceArtboard, _super);
    function ViewModelInstanceArtboard(instance, root) {
        return _super.call(this, instance, root) || this;
    }
    Object.defineProperty(ViewModelInstanceArtboard.prototype, "value", {
        set: function (artboard) {
            var _a, _b;
            var bindableArtboard;
            if (artboard.isBindableArtboard) {
                bindableArtboard = artboard;
            }
            else {
                bindableArtboard = artboard.file.internalBindableArtboardFromArtboard(artboard.nativeArtboard);
            }
            this._viewModelInstanceValue.value((_a = bindableArtboard === null || bindableArtboard === void 0 ? void 0 : bindableArtboard.nativeArtboard) !== null && _a !== void 0 ? _a : null);
            if (bindableArtboard === null || bindableArtboard === void 0 ? void 0 : bindableArtboard.nativeViewModel) {
                this._viewModelInstanceValue.viewModelInstance((_b = bindableArtboard === null || bindableArtboard === void 0 ? void 0 : bindableArtboard.nativeViewModel) !== null && _b !== void 0 ? _b : null);
            }
        },
        enumerable: false,
        configurable: true
    });
    ViewModelInstanceArtboard.prototype.internalHandleCallback = function (callback) {
        callback();
    };
    return ViewModelInstanceArtboard;
}(ViewModelInstanceValue));

// Loads Rive data from a URI via fetch.
var loadRiveFile = function (src) { return __awaiter(void 0, void 0, void 0, function () {
    var req, res, buffer;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                req = new Request(src);
                return [4 /*yield*/, fetch(req)];
            case 1:
                res = _a.sent();
                return [4 /*yield*/, res.arrayBuffer()];
            case 2:
                buffer = _a.sent();
                return [2 /*return*/, buffer];
        }
    });
}); };
// #endregion
// #region utility functions
/*
 * Utility function to ensure an object is a string array
 */
var mapToStringArray = function (obj) {
    if (typeof obj === "string") {
        return [obj];
    }
    else if (obj instanceof Array) {
        return obj;
    }
    // If obj is undefined, return empty array
    return [];
};
// #endregion
// #region testing utilities
// Exports to only be used for tests
var Testing = {
    EventManager: EventManager,
    TaskQueueManager: TaskQueueManager,
};
// #endregion
// #region asset loaders
/**
 * Decodes bytes into an audio asset.
 *
 * Be sure to call `.unref()` on the audio once it is no longer needed. This
 * allows the engine to clean it up when it is not used by any more animations.
 */
var decodeAudio = function (bytes) { return __awaiter(void 0, void 0, void 0, function () {
    var decodedPromise, audio, audioWrapper;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                decodedPromise = new Promise(function (resolve) {
                    return RuntimeLoader.getInstance(function (rive) {
                        rive.decodeAudio(bytes, resolve);
                    });
                });
                return [4 /*yield*/, decodedPromise];
            case 1:
                audio = _a.sent();
                audioWrapper = new _utils__WEBPACK_IMPORTED_MODULE_3__.AudioWrapper(audio);
                _utils__WEBPACK_IMPORTED_MODULE_3__.finalizationRegistry.register(audioWrapper, audio);
                return [2 /*return*/, audioWrapper];
        }
    });
}); };
/**
 * Decodes bytes into an image.
 *
 * Be sure to call `.unref()` on the image once it is no longer needed. This
 * allows the engine to clean it up when it is not used by any more animations.
 */
var decodeImage = function (bytes) { return __awaiter(void 0, void 0, void 0, function () {
    var decodedPromise, image, imageWrapper;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                decodedPromise = new Promise(function (resolve) {
                    return RuntimeLoader.getInstance(function (rive) {
                        rive.decodeImage(bytes, resolve);
                    });
                });
                return [4 /*yield*/, decodedPromise];
            case 1:
                image = _a.sent();
                imageWrapper = new _utils__WEBPACK_IMPORTED_MODULE_3__.ImageWrapper(image);
                _utils__WEBPACK_IMPORTED_MODULE_3__.finalizationRegistry.register(imageWrapper, image);
                return [2 /*return*/, imageWrapper];
        }
    });
}); };
/**
 * Decodes bytes into a font.
 *
 * Be sure to call `.unref()` on the font once it is no longer needed. This
 * allows the engine to clean it up when it is not used by any more animations.
 */
var decodeFont = function (bytes) { return __awaiter(void 0, void 0, void 0, function () {
    var decodedPromise, font, fontWrapper;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                decodedPromise = new Promise(function (resolve) {
                    return RuntimeLoader.getInstance(function (rive) {
                        rive.decodeFont(bytes, resolve);
                    });
                });
                return [4 /*yield*/, decodedPromise];
            case 1:
                font = _a.sent();
                fontWrapper = new _utils__WEBPACK_IMPORTED_MODULE_3__.FontWrapper(font);
                _utils__WEBPACK_IMPORTED_MODULE_3__.finalizationRegistry.register(fontWrapper, font);
                return [2 /*return*/, fontWrapper];
        }
    });
}); };
// #endregion

})();

/******/ 	return __webpack_exports__;
/******/ })()
;
});
//# sourceMappingURL=rive.js.map