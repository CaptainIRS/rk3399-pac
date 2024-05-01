#[doc = "Register `WB_CTRL1` reader"]
pub type R = crate::R<WbCtrl1Spec>;
#[doc = "Register `WB_CTRL1` writer"]
pub type W = crate::W<WbCtrl1Spec>;
#[doc = "Field `WB_XPSD_BIL_FACTOR` reader - factor=((src_width\\[11:0\\])/(dst_width\\[11:0\\]))*2^12"]
pub type WbXpsdBilFactorR = crate::FieldReader<u16>;
#[doc = "Field `WB_XPSD_BIL_FACTOR` writer - factor=((src_width\\[11:0\\])/(dst_width\\[11:0\\]))*2^12"]
pub type WbXpsdBilFactorW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 16:29 - factor=((src_width\\[11:0\\])/(dst_width\\[11:0\\]))*2^12"]
    #[inline(always)]
    pub fn wb_xpsd_bil_factor(&self) -> WbXpsdBilFactorR {
        WbXpsdBilFactorR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - factor=((src_width\\[11:0\\])/(dst_width\\[11:0\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn wb_xpsd_bil_factor(&mut self) -> WbXpsdBilFactorW<WbCtrl1Spec> {
        WbXpsdBilFactorW::new(self, 16)
    }
}
#[doc = "write back ctrl1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WbCtrl1Spec;
impl crate::RegisterSpec for WbCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wb_ctrl1::R`](R) reader structure"]
impl crate::Readable for WbCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`wb_ctrl1::W`](W) writer structure"]
impl crate::Writable for WbCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WB_CTRL1 to value 0"]
impl crate::Resettable for WbCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
