import { Banner } from "banner-of-life";

const banner = Banner.new("banner-of-life-canvas");
banner.setCellSize(8);
banner.setFontSize(60);
banner.setBackgroundColor("White");
banner.setCellColor("Black");
banner.setGridColor("#DCDCDC");

banner.render("R");

document.addEventListener("click", event => {
  banner.tick();
});

