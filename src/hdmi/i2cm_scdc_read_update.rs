#[doc = "Register `I2CM_SCDC_READ_UPDATE` reader"]
pub type R = crate::R<I2cmScdcReadUpdateSpec>;
#[doc = "Register `I2CM_SCDC_READ_UPDATE` writer"]
pub type W = crate::W<I2cmScdcReadUpdateSpec>;
#[doc = "Field `READ_UPDATE` writer - When set to 1'b1, a SCDC Update Read is performed\n\nand the read data loaded into registers\n\ni2cm_scdc_update0 and i2cm_scdc_update1."]
pub type ReadUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_REQUEST_EN` reader - Read request enabled. When active (1'b1) an SCDC\n\nUpdate Read shall be performed whenever a SCDC\n\nread request is detected."]
pub type ReadRequestEnR = crate::BitReader;
#[doc = "Field `READ_REQUEST_EN` writer - Read request enabled. When active (1'b1) an SCDC\n\nUpdate Read shall be performed whenever a SCDC\n\nread request is detected."]
pub type ReadRequestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDTRD_VSYNCPOLL_EN` reader - Update read polling enabled. When active (1'b1), an\n\nSCDC Update Read is performed at the fall of the\n\nactive edge of the vertical sync pulse."]
pub type UpdtrdVsyncpollEnR = crate::BitReader;
#[doc = "Field `UPDTRD_VSYNCPOLL_EN` writer - Update read polling enabled. When active (1'b1), an\n\nSCDC Update Read is performed at the fall of the\n\nactive edge of the vertical sync pulse."]
pub type UpdtrdVsyncpollEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Read request enabled. When active (1'b1) an SCDC\n\nUpdate Read shall be performed whenever a SCDC\n\nread request is detected."]
    #[inline(always)]
    pub fn read_request_en(&self) -> ReadRequestEnR {
        ReadRequestEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Update read polling enabled. When active (1'b1), an\n\nSCDC Update Read is performed at the fall of the\n\nactive edge of the vertical sync pulse."]
    #[inline(always)]
    pub fn updtrd_vsyncpoll_en(&self) -> UpdtrdVsyncpollEnR {
        UpdtrdVsyncpollEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1'b1, a SCDC Update Read is performed\n\nand the read data loaded into registers\n\ni2cm_scdc_update0 and i2cm_scdc_update1."]
    #[inline(always)]
    #[must_use]
    pub fn read_update(&mut self) -> ReadUpdateW<I2cmScdcReadUpdateSpec> {
        ReadUpdateW::new(self, 0)
    }
    #[doc = "Bit 4 - Read request enabled. When active (1'b1) an SCDC\n\nUpdate Read shall be performed whenever a SCDC\n\nread request is detected."]
    #[inline(always)]
    #[must_use]
    pub fn read_request_en(&mut self) -> ReadRequestEnW<I2cmScdcReadUpdateSpec> {
        ReadRequestEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Update read polling enabled. When active (1'b1), an\n\nSCDC Update Read is performed at the fall of the\n\nactive edge of the vertical sync pulse."]
    #[inline(always)]
    #[must_use]
    pub fn updtrd_vsyncpoll_en(&mut self) -> UpdtrdVsyncpollEnW<I2cmScdcReadUpdateSpec> {
        UpdtrdVsyncpollEnW::new(self, 5)
    }
}
#[doc = "SCDC Control Register\n\nThis register configures the SCDC update status read through the I2C master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_scdc_read_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_scdc_read_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmScdcReadUpdateSpec;
impl crate::RegisterSpec for I2cmScdcReadUpdateSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_scdc_read_update::R`](R) reader structure"]
impl crate::Readable for I2cmScdcReadUpdateSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_scdc_read_update::W`](W) writer structure"]
impl crate::Writable for I2cmScdcReadUpdateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_SCDC_READ_UPDATE to value 0"]
impl crate::Resettable for I2cmScdcReadUpdateSpec {
    const RESET_VALUE: u8 = 0;
}
