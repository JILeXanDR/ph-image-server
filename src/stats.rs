use crate::models::advertisement::Advertisement;

pub struct ShowStatistic {
    pub uid: i64,
    pub cid: i64,
    pub os: i8,
    pub browser: i8,
    pub country: i64,
    pub sub_acc: i64,
    pub operator: i64,
    pub adv_type: Advertisement,
    pub device: i8,
}

pub fn increment(_stats: ShowStatistic) {
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
