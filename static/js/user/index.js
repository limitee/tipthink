var Com = function(config) {
    var self = this;
    self.config = config;
    //子页面
    self.cr = {
        'main': {
            pins: self,
            parent: $('#content')
        }
    };
    self.init();
};

Com.prototype.init = function() {
    var self = this;
    self.dom_my_files = $('#user_info');
    self.dom_my_files.on('click', function(e) {
        CurSite.to_page(self.cr.main, "user_info");
    });
    
    CurSite.to_page(self.cr.main, "user_info");
};

return Com;
