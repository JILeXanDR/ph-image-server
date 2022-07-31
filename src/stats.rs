pub struct ShowStatistic {
    pub uid: i8,
    pub cid: i8,
    pub os: i8,
    pub browser: i8,
    pub country: i8,
    pub sub_acc: i8,
    pub operator: i8,
    pub adv_type: i8,
    pub device: i8,
}

pub fn increment(stats: ShowStatistic) {
    // TODO: write stats.
    // go func() {
    //     err := w.stats.Increment(service.ShowStatistic{
    //         UID:      icon.UID,
    //         CID:      icon.CID,
    //         OS:       os,
    //         Browser:  browser,
    //         Country:  icon.Country,
    //         SubAcc:   icon.SubAcc,
    //         Operator: icon.Operator,
    //         AdvType:  icon.AdvertisementType,
    //         Device:   device,
    //     })
    //     if err != nil {
    //         log.Info("cannot increment stats", zap.Error(err))
    //     }
    // }()
}
