#[doc = "Register `FLASH_DELAY` reader"]
pub type R = crate::R<FlashDelaySpec>;
#[doc = "Register `FLASH_DELAY` writer"]
pub type W = crate::W<FlashDelaySpec>;
#[doc = "Field `fl_delay` reader - counter value for flash/preflash delay open_delay = (fl_delay + 1) * (fl_pre_div+1) / clk_isp fl_delay = (open_delay * clk_isp) / (fl_pre_div+1) - 1"]
pub type FlDelayR = crate::FieldReader<u32>;
#[doc = "Field `fl_delay` writer - counter value for flash/preflash delay open_delay = (fl_delay + 1) * (fl_pre_div+1) / clk_isp fl_delay = (open_delay * clk_isp) / (fl_pre_div+1) - 1"]
pub type FlDelayW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - counter value for flash/preflash delay open_delay = (fl_delay + 1) * (fl_pre_div+1) / clk_isp fl_delay = (open_delay * clk_isp) / (fl_pre_div+1) - 1"]
    #[inline(always)]
    pub fn fl_delay(&self) -> FlDelayR {
        FlDelayR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - counter value for flash/preflash delay open_delay = (fl_delay + 1) * (fl_pre_div+1) / clk_isp fl_delay = (open_delay * clk_isp) / (fl_pre_div+1) - 1"]
    #[inline(always)]
    #[must_use]
    pub fn fl_delay(&mut self) -> FlDelayW<FlashDelaySpec> {
        FlDelayW::new(self, 0)
    }
}
#[doc = "Flash Delay\n\nNote: Example: \n\nfl_delay = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashDelaySpec;
impl crate::RegisterSpec for FlashDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_delay::R`](R) reader structure"]
impl crate::Readable for FlashDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`flash_delay::W`](W) writer structure"]
impl crate::Writable for FlashDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_DELAY to value 0"]
impl crate::Resettable for FlashDelaySpec {
    const RESET_VALUE: u32 = 0;
}
