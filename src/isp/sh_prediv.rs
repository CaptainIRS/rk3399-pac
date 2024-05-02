#[doc = "Register `SH_PREDIV` reader"]
pub type R = crate::R<ShPredivSpec>;
#[doc = "Register `SH_PREDIV` writer"]
pub type W = crate::W<ShPredivSpec>;
#[doc = "Field `sh_pre_div` reader - pre-divider for mechanical shutter open_delay and open_time counter\n\n"]
pub type ShPreDivR = crate::FieldReader<u16>;
#[doc = "Field `sh_pre_div` writer - pre-divider for mechanical shutter open_delay and open_time counter\n\n"]
pub type ShPreDivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - pre-divider for mechanical shutter open_delay and open_time counter\n\n"]
    #[inline(always)]
    pub fn sh_pre_div(&self) -> ShPreDivR {
        ShPreDivR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - pre-divider for mechanical shutter open_delay and open_time counter\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn sh_pre_div(&mut self) -> ShPreDivW<ShPredivSpec> {
        ShPreDivW::new(self, 0)
    }
}
#[doc = "Mech. Shutter Counter Pre-Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sh_prediv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sh_prediv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShPredivSpec;
impl crate::RegisterSpec for ShPredivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sh_prediv::R`](R) reader structure"]
impl crate::Readable for ShPredivSpec {}
#[doc = "`write(|w| ..)` method takes [`sh_prediv::W`](W) writer structure"]
impl crate::Writable for ShPredivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SH_PREDIV to value 0"]
impl crate::Resettable for ShPredivSpec {
    const RESET_VALUE: u32 = 0;
}
