#[doc = "Register `POWER_ST` reader"]
pub type R = crate::R<PowerStSpec>;
#[doc = "Register `POWER_ST` writer"]
pub type W = crate::W<PowerStSpec>;
#[doc = "Field `POWER_STATE` reader - power state of pmu FSM"]
pub type PowerStateR = crate::FieldReader;
#[doc = "Field `POWER_STATE` writer - power state of pmu FSM"]
pub type PowerStateW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - power state of pmu FSM"]
    #[inline(always)]
    pub fn power_state(&self) -> PowerStateR {
        PowerStateR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - power state of pmu FSM"]
    #[inline(always)]
    #[must_use]
    pub fn power_state(&mut self) -> PowerStateW<PowerStSpec> {
        PowerStateW::new(self, 0)
    }
}
#[doc = "pmu power status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerStSpec;
impl crate::RegisterSpec for PowerStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_st::R`](R) reader structure"]
impl crate::Readable for PowerStSpec {}
#[doc = "`write(|w| ..)` method takes [`power_st::W`](W) writer structure"]
impl crate::Writable for PowerStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_ST to value 0"]
impl crate::Resettable for PowerStSpec {
    const RESET_VALUE: u32 = 0;
}
