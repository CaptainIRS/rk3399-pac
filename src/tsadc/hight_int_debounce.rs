#[doc = "Register `HIGHT_INT_DEBOUNCE` reader"]
pub type R = crate::R<HightIntDebounceSpec>;
#[doc = "Register `HIGHT_INT_DEBOUNCE` writer"]
pub type W = crate::W<HightIntDebounceSpec>;
#[doc = "Field `DEBOUNCE` reader - TSADC controller will only generate interrupt or TSHUT when\n\ntemperature is higher than COMP_INT for 'debounce' times."]
pub type DebounceR = crate::FieldReader;
#[doc = "Field `DEBOUNCE` writer - TSADC controller will only generate interrupt or TSHUT when\n\ntemperature is higher than COMP_INT for 'debounce' times."]
pub type DebounceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TSADC controller will only generate interrupt or TSHUT when\n\ntemperature is higher than COMP_INT for 'debounce' times."]
    #[inline(always)]
    pub fn debounce(&self) -> DebounceR {
        DebounceR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TSADC controller will only generate interrupt or TSHUT when\n\ntemperature is higher than COMP_INT for 'debounce' times."]
    #[inline(always)]
    #[must_use]
    pub fn debounce(&mut self) -> DebounceW<HightIntDebounceSpec> {
        DebounceW::new(self, 0)
    }
}
#[doc = "high temperature debounce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hight_int_debounce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hight_int_debounce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HightIntDebounceSpec;
impl crate::RegisterSpec for HightIntDebounceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hight_int_debounce::R`](R) reader structure"]
impl crate::Readable for HightIntDebounceSpec {}
#[doc = "`write(|w| ..)` method takes [`hight_int_debounce::W`](W) writer structure"]
impl crate::Writable for HightIntDebounceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIGHT_INT_DEBOUNCE to value 0x03"]
impl crate::Resettable for HightIntDebounceSpec {
    const RESET_VALUE: u32 = 0x03;
}
