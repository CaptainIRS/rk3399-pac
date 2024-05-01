#[doc = "Register `SWREG61` reader"]
pub type R = crate::R<Swreg61Spec>;
#[doc = "Register `SWREG61` writer"]
pub type W = crate::W<Swreg61Spec>;
#[doc = "Field `SW_QTABLE_ST_ADR` reader - standard dependent tables start address\n\nJPEG : AC,DC, QP tables\n\nMPEG4/2 : QP table\n\nH.264 :various tables\n\nVP6/7/8 : stream decoding tables"]
pub type SwQtableStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_QTABLE_ST_ADR` writer - standard dependent tables start address\n\nJPEG : AC,DC, QP tables\n\nMPEG4/2 : QP table\n\nH.264 :various tables\n\nVP6/7/8 : stream decoding tables"]
pub type SwQtableStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - standard dependent tables start address\n\nJPEG : AC,DC, QP tables\n\nMPEG4/2 : QP table\n\nH.264 :various tables\n\nVP6/7/8 : stream decoding tables"]
    #[inline(always)]
    pub fn sw_qtable_st_adr(&self) -> SwQtableStAdrR {
        SwQtableStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - standard dependent tables start address\n\nJPEG : AC,DC, QP tables\n\nMPEG4/2 : QP table\n\nH.264 :various tables\n\nVP6/7/8 : stream decoding tables"]
    #[inline(always)]
    #[must_use]
    pub fn sw_qtable_st_adr(&mut self) -> SwQtableStAdrW<Swreg61Spec> {
        SwQtableStAdrW::new(self, 2)
    }
}
#[doc = "standard dependent tables start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg61::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg61::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg61Spec;
impl crate::RegisterSpec for Swreg61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg61::R`](R) reader structure"]
impl crate::Readable for Swreg61Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg61::W`](W) writer structure"]
impl crate::Writable for Swreg61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG61 to value 0"]
impl crate::Resettable for Swreg61Spec {
    const RESET_VALUE: u32 = 0;
}
