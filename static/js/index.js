var Com = function(config) {
    var self = this;
    self.config = config;
    //子页面
    self.cr = {
        'main': {
            pins: self,
            parent: $("#content")
        }
    };
    self.init();
};

Com.prototype.init = function() {
    var self = this;
    CurSite.postUnDigest({cmd:"GBT01"}, {}, function(err, data){
        console.log(data);
        if(data) {
        }
    });
};

return Com;
