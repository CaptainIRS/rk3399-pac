#[doc = "Register `WB_CBR_MST` reader"]
pub type R = crate::R<WbCbrMstSpec>;
#[doc = "Register `WB_CBR_MST` writer"]
pub type W = crate::W<WbCbrMstSpec>;
#[doc = "Field `WB_CBR_MST` reader - CBR mst adress"]
pub type WbCbrMstR = crate::FieldReader<u32>;
#[doc = "Field `WB_CBR_MST` writer - CBR mst adress"]
pub type WbCbrMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CBR mst adress"]
    #[inline(always)]
    pub fn wb_cbr_mst(&self) -> WbCbrMstR {
        WbCbrMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CBR mst adress"]
    #[inline(always)]
    #[must_use]
    pub fn wb_cbr_mst(&mut self) -> WbCbrMstW<WbCbrMstSpec> {
        WbCbrMstW::new(self, 0)
    }
}
#[doc = "write back cbr mst\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_cbr_mst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_cbr_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WbCbrMstSpec;
impl crate::RegisterSpec for WbCbrMstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wb_cbr_mst::R`](R) reader structure"]
impl crate::Readable for WbCbrMstSpec {}
#[doc = "`write(|w| ..)` method takes [`wb_cbr_mst::W`](W) writer structure"]
impl crate::Writable for WbCbrMstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WB_CBR_MST to value 0"]
impl crate::Resettable for WbCbrMstSpec {
    const RESET_VALUE: u32 = 0;
}
