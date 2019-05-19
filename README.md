banner-of-life
==============

A banner to show strings as a game of life pattern on HTML `<canvas>` element.

Install
-------

```shell
npm install banner-of-life
```

Getting started
---------------

```javascript
import { Banner } from "banner-of-life";

const banner = Banner.new("canvas-id-to-display-strings");

banner.setCellSize(10);
banner.setFontSize(60);
banner.setBackgroundColor("White");
banner.setCellColor("Black");
banner.setGridColor("#DCDCDC");

banner.render("Rust");
banner.tick();
```
