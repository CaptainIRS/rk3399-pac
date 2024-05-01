#[doc = "Register `WIN1_CBR_MST` reader"]
pub type R = crate::R<Win1CbrMstSpec>;
#[doc = "Register `WIN1_CBR_MST` writer"]
pub type W = crate::W<Win1CbrMstSpec>;
#[doc = "Field `WIN1_CBR_MST` reader - win1 CBR frame buffer memory start address"]
pub type Win1CbrMstR = crate::FieldReader<u32>;
#[doc = "Field `WIN1_CBR_MST` writer - win1 CBR frame buffer memory start address"]
pub type Win1CbrMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - win1 CBR frame buffer memory start address"]
    #[inline(always)]
    pub fn win1_cbr_mst(&self) -> Win1CbrMstR {
        Win1CbrMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - win1 CBR frame buffer memory start address"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_mst(&mut self) -> Win1CbrMstW<Win1CbrMstSpec> {
        Win1CbrMstW::new(self, 0)
    }
}
#[doc = "Win1 Cbr memory start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_cbr_mst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_cbr_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1CbrMstSpec;
impl crate::RegisterSpec for Win1CbrMstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_cbr_mst::R`](R) reader structure"]
impl crate::Readable for Win1CbrMstSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_cbr_mst::W`](W) writer structure"]
impl crate::Writable for Win1CbrMstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_CBR_MST to value 0"]
impl crate::Resettable for Win1CbrMstSpec {
    const RESET_VALUE: u32 = 0;
}
