#[doc = "Register `SH_DELAY` reader"]
pub type R = crate::R<ShDelaySpec>;
#[doc = "Register `SH_DELAY` writer"]
pub type W = crate::W<ShDelaySpec>;
#[doc = "Field `sh_delay` reader - counter value for delay open_delay = (sh_delay + 1) * (fl_pre_div+1) / clk_isp sh_delay = (open_delay\n\n* clk_isp) / (sh_pre_div+1) – 1"]
pub type ShDelayR = crate::FieldReader<u32>;
#[doc = "Field `sh_delay` writer - counter value for delay open_delay = (sh_delay + 1) * (fl_pre_div+1) / clk_isp sh_delay = (open_delay\n\n* clk_isp) / (sh_pre_div+1) – 1"]
pub type ShDelayW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - counter value for delay open_delay = (sh_delay + 1) * (fl_pre_div+1) / clk_isp sh_delay = (open_delay\n\n* clk_isp) / (sh_pre_div+1) – 1"]
    #[inline(always)]
    pub fn sh_delay(&self) -> ShDelayR {
        ShDelayR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - counter value for delay open_delay = (sh_delay + 1) * (fl_pre_div+1) / clk_isp sh_delay = (open_delay\n\n* clk_isp) / (sh_pre_div+1) – 1"]
    #[inline(always)]
    #[must_use]
    pub fn sh_delay(&mut self) -> ShDelayW<ShDelaySpec> {
        ShDelayW::new(self, 0)
    }
}
#[doc = "Delay register\n\nNote: Example: \n\nsh_delay = (250us * 100MHz) / (50 + 1) – 1 = 489 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShDelaySpec;
impl crate::RegisterSpec for ShDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sh_delay::R`](R) reader structure"]
impl crate::Readable for ShDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`sh_delay::W`](W) writer structure"]
impl crate::Writable for ShDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SH_DELAY to value 0"]
impl crate::Resettable for ShDelaySpec {
    const RESET_VALUE: u32 = 0;
}
