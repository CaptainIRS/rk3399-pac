#[doc = "Register `FLASH_TIME` reader"]
pub type R = crate::R<FlashTimeSpec>;
#[doc = "Register `FLASH_TIME` writer"]
pub type W = crate::W<FlashTimeSpec>;
#[doc = "Field `fl_time` reader - counter value for flash/preflash time open_time = (fl_time + 1) * (fl_pre_div+1) / clk_isp fl_time = (open_time * clk_isp) / (fl_pre_div+1) - 1"]
pub type FlTimeR = crate::FieldReader<u32>;
#[doc = "Field `fl_time` writer - counter value for flash/preflash time open_time = (fl_time + 1) * (fl_pre_div+1) / clk_isp fl_time = (open_time * clk_isp) / (fl_pre_div+1) - 1"]
pub type FlTimeW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - counter value for flash/preflash time open_time = (fl_time + 1) * (fl_pre_div+1) / clk_isp fl_time = (open_time * clk_isp) / (fl_pre_div+1) - 1"]
    #[inline(always)]
    pub fn fl_time(&self) -> FlTimeR {
        FlTimeR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - counter value for flash/preflash time open_time = (fl_time + 1) * (fl_pre_div+1) / clk_isp fl_time = (open_time * clk_isp) / (fl_pre_div+1) - 1"]
    #[inline(always)]
    #[must_use]
    pub fn fl_time(&mut self) -> FlTimeW<FlashTimeSpec> {
        FlTimeW::new(self, 0)
    }
}
#[doc = "Flash time\n\nNote: Example: \n\nfl_time = (500ms * 100MHz) / (700 + 1) – 1 = 71530 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashTimeSpec;
impl crate::RegisterSpec for FlashTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_time::R`](R) reader structure"]
impl crate::Readable for FlashTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_time::W`](W) writer structure"]
impl crate::Writable for FlashTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_TIME to value 0"]
impl crate::Resettable for FlashTimeSpec {
    const RESET_VALUE: u32 = 0;
}
