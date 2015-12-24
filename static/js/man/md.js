var Com = function(config) {
    var self = this;
    self.config = config;
    //子页面
    self.cr = {
    };
    self.init();
}

Com.prototype.init = function() {
    var self = this;
    self.scroll_mode = 0;

    self.dom_infer_frame = $("#infer_frame");

    self.dom_infer = $("#infer");
    self.dom_infer.on("keydown", function(e) {
        if(e.keyCode == 13) {
            var url = $(this).val();
            console.log(url);
            var html = '<iframe src="' + url + '" width="100%" height="260px" frameborder="no" border="0"></iframe>'
            self.dom_infer_frame.html(html);
        }
    })

    self.dom_mode_panel = $("#mode_panel");
    self.dom_mode_scroll = $("#mode_scroll"); 
    self.dom_mode_free = $("#mode_free"); 
    self.dom_input = $("#text_input");
    self.dom_preview = $("#preview");
    self.update();

    self.dom_mode_scroll.on("click", function(e) {
        self.dom_mode_panel.html("滚动");
        self.scroll_mode = 0;
        e.preventDefault();
    });

    self.dom_mode_free.on("click", function(e) {
        self.dom_mode_panel.html("自由");
        self.scroll_mode = 1;
        e.preventDefault();
    });

    self.dom_input.on("input", function() {
        self.update();
    });
}

Com.prototype.update = function() {
    var self = this;
    self.dom_preview.html(mk.to_html(self.dom_input.val()));
    if(self.scroll_mode == 0) {
        var mydiv = $('#main');
        mydiv.scrollTop(mydiv.prop('scrollHeight'));
    }
}

return Com;
