#[doc = "Register `SH_TIME` reader"]
pub type R = crate::R<ShTimeSpec>;
#[doc = "Register `SH_TIME` writer"]
pub type W = crate::W<ShTimeSpec>;
#[doc = "Field `sh_time` reader - counter value for time open_time = (sh_time + 1) * (fl_pre_div+1) / clk_isp sh_time = (open_time\n\n* clk_isp) / (sh_pre_div+1) - 1"]
pub type ShTimeR = crate::FieldReader<u32>;
#[doc = "Field `sh_time` writer - counter value for time open_time = (sh_time + 1) * (fl_pre_div+1) / clk_isp sh_time = (open_time\n\n* clk_isp) / (sh_pre_div+1) - 1"]
pub type ShTimeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - counter value for time open_time = (sh_time + 1) * (fl_pre_div+1) / clk_isp sh_time = (open_time\n\n* clk_isp) / (sh_pre_div+1) - 1"]
    #[inline(always)]
    pub fn sh_time(&self) -> ShTimeR {
        ShTimeR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - counter value for time open_time = (sh_time + 1) * (fl_pre_div+1) / clk_isp sh_time = (open_time\n\n* clk_isp) / (sh_pre_div+1) - 1"]
    #[inline(always)]
    #[must_use]
    pub fn sh_time(&mut self) -> ShTimeW<ShTimeSpec> {
        ShTimeW::new(self, 0)
    }
}
#[doc = "Time register\n\nNote: Example: \n\n\n\nsh_time = (10s * 100MHz) / (1023 + 1) – 1 = 976561 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShTimeSpec;
impl crate::RegisterSpec for ShTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sh_time::R`](R) reader structure"]
impl crate::Readable for ShTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`sh_time::W`](W) writer structure"]
impl crate::Writable for ShTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SH_TIME to value 0"]
impl crate::Resettable for ShTimeSpec {
    const RESET_VALUE: u32 = 0;
}
