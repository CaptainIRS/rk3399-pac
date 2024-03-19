#[doc = "Register `DDR_DENALI_CTL_126` reader"]
pub type R = crate::R<DdrDenaliCtl126Spec>;
#[doc = "Register `DDR_DENALI_CTL_126` writer"]
pub type W = crate::W<DdrDenaliCtl126Spec>;
#[doc = "Field `TFC_F1` reader - JEDEC TFC, the frequency set point switching time."]
pub type TfcF1R = crate::FieldReader<u16>;
#[doc = "Field `TFC_F1` writer - JEDEC TFC, the frequency set point switching time."]
pub type TfcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TCKFSPE_F1` reader - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
pub type TckfspeF1R = crate::FieldReader;
#[doc = "Field `TCKFSPE_F1` writer - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
pub type TckfspeF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKFSPX_F1` reader - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
pub type TckfspxF1R = crate::FieldReader;
#[doc = "Field `TCKFSPX_F1` writer - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
pub type TckfspxF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - JEDEC TFC, the frequency set point switching time."]
    #[inline(always)]
    pub fn tfc_f1(&self) -> TfcF1R {
        TfcF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
    #[inline(always)]
    pub fn tckfspe_f1(&self) -> TckfspeF1R {
        TckfspeF1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
    #[inline(always)]
    pub fn tckfspx_f1(&self) -> TckfspxF1R {
        TckfspxF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - JEDEC TFC, the frequency set point switching time."]
    #[inline(always)]
    #[must_use]
    pub fn tfc_f1(&mut self) -> TfcF1W<DdrDenaliCtl126Spec> {
        TfcF1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
    #[inline(always)]
    #[must_use]
    pub fn tckfspe_f1(&mut self) -> TckfspeF1W<DdrDenaliCtl126Spec> {
        TckfspeF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
    #[inline(always)]
    #[must_use]
    pub fn tckfspx_f1(&mut self) -> TckfspxF1W<DdrDenaliCtl126Spec> {
        TckfspxF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_126::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_126::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl126Spec;
impl crate::RegisterSpec for DdrDenaliCtl126Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_126::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl126Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_126::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl126Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_126 to value 0"]
impl crate::Resettable for DdrDenaliCtl126Spec {
    const RESET_VALUE: u32 = 0;
}
