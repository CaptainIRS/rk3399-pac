#[doc = "Register `SWREG_80` reader"]
pub type R = crate::R<Swreg80Spec>;
#[doc = "Register `SWREG_80` writer"]
pub type W = crate::W<Swreg80Spec>;
#[doc = "Field `MV_OUT_ST_ADR` reader - MV wr start address"]
pub type MvOutStAdrR = crate::FieldReader<u32>;
#[doc = "Field `MV_OUT_ST_ADR` writer - MV wr start address"]
pub type MvOutStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MV wr start address"]
    #[inline(always)]
    pub fn mv_out_st_adr(&self) -> MvOutStAdrR {
        MvOutStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MV wr start address"]
    #[inline(always)]
    #[must_use]
    pub fn mv_out_st_adr(&mut self) -> MvOutStAdrW<Swreg80Spec> {
        MvOutStAdrW::new(self, 0)
    }
}
#[doc = "Base address for MV output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_80::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_80::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg80Spec;
impl crate::RegisterSpec for Swreg80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_80::R`](R) reader structure"]
impl crate::Readable for Swreg80Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_80::W`](W) writer structure"]
impl crate::Writable for Swreg80Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_80 to value 0"]
impl crate::Resettable for Swreg80Spec {
    const RESET_VALUE: u32 = 0;
}
