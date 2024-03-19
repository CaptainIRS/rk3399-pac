#[doc = "Register `SDMMC_TMOUT` reader"]
pub type R = crate::R<SdmmcTmoutSpec>;
#[doc = "Register `SDMMC_TMOUT` writer"]
pub type W = crate::W<SdmmcTmoutSpec>;
#[doc = "Field `RESPONSE_TIMEOUT` reader - Response timeout value.\n\nValue is in number of card output clocks –cclk_out."]
pub type ResponseTimeoutR = crate::FieldReader;
#[doc = "Field `RESPONSE_TIMEOUT` writer - Response timeout value.\n\nValue is in number of card output clocks –cclk_out."]
pub type ResponseTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA_TIMEOUT` reader - Value for card Data Read Timeout; same value also used for Data\n\nStarvation by Host timeout.\n\nValue is in number of card output clocks cclk_out of selected\n\ncard.\n\nNote: The software timer should be used if the timeout value is in\n\nthe order of 100 ms. In this case, read data timeout interrupt\n\nneeds to be disabled."]
pub type DataTimeoutR = crate::FieldReader<u32>;
#[doc = "Field `DATA_TIMEOUT` writer - Value for card Data Read Timeout; same value also used for Data\n\nStarvation by Host timeout.\n\nValue is in number of card output clocks cclk_out of selected\n\ncard.\n\nNote: The software timer should be used if the timeout value is in\n\nthe order of 100 ms. In this case, read data timeout interrupt\n\nneeds to be disabled."]
pub type DataTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - Response timeout value.\n\nValue is in number of card output clocks –cclk_out."]
    #[inline(always)]
    pub fn response_timeout(&self) -> ResponseTimeoutR {
        ResponseTimeoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Value for card Data Read Timeout; same value also used for Data\n\nStarvation by Host timeout.\n\nValue is in number of card output clocks cclk_out of selected\n\ncard.\n\nNote: The software timer should be used if the timeout value is in\n\nthe order of 100 ms. In this case, read data timeout interrupt\n\nneeds to be disabled."]
    #[inline(always)]
    pub fn data_timeout(&self) -> DataTimeoutR {
        DataTimeoutR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Response timeout value.\n\nValue is in number of card output clocks –cclk_out."]
    #[inline(always)]
    #[must_use]
    pub fn response_timeout(&mut self) -> ResponseTimeoutW<SdmmcTmoutSpec> {
        ResponseTimeoutW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Value for card Data Read Timeout; same value also used for Data\n\nStarvation by Host timeout.\n\nValue is in number of card output clocks cclk_out of selected\n\ncard.\n\nNote: The software timer should be used if the timeout value is in\n\nthe order of 100 ms. In this case, read data timeout interrupt\n\nneeds to be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout(&mut self) -> DataTimeoutW<SdmmcTmoutSpec> {
        DataTimeoutW::new(self, 8)
    }
}
#[doc = "Time-out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_tmout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_tmout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcTmoutSpec;
impl crate::RegisterSpec for SdmmcTmoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_tmout::R`](R) reader structure"]
impl crate::Readable for SdmmcTmoutSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_tmout::W`](W) writer structure"]
impl crate::Writable for SdmmcTmoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_TMOUT to value 0xffff_ff40"]
impl crate::Resettable for SdmmcTmoutSpec {
    const RESET_VALUE: u32 = 0xffff_ff40;
}
