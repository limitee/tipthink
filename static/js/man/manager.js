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
    self.dom_my_files = $('#my_files');
    self.dom_my_files.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_file_list");
    });
    
    self.dom_book_type = $('#book_type');
    self.dom_book_type.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_book_typelist");
    });

    self.dom_my_files = $('#upload_files');
    self.dom_my_files.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_file_upload");
    });

    self.dom_my_files = $('#customer');
    self.dom_my_files.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_user_list");
    });
    CurSite.to_page(self.cr.main, "man_user_list");
};

return Com;
