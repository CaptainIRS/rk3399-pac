#[doc = "Register `PIPE_CMN_CTRL2` reader"]
pub type R = crate::R<PipeCmnCtrl2Spec>;
#[doc = "Register `PIPE_CMN_CTRL2` writer"]
pub type W = crate::W<PipeCmnCtrl2Spec>;
#[doc = "Field `FIELD1` reader - Reserved"]
pub type Field1R = crate::FieldReader<u16>;
#[doc = "Field `FIELD0` reader - USB SuperSpeed TX LFPS Stretch : Minimum number of data rate \n\nclock cycles in which PMA tx_lfps_en signal is asserted. Number of \n\ndata rate clock cycles must be > 1 PMA RefClk cycle."]
pub type Field0R = crate::FieldReader;
#[doc = "Field `FIELD0` writer - USB SuperSpeed TX LFPS Stretch : Minimum number of data rate \n\nclock cycles in which PMA tx_lfps_en signal is asserted. Number of \n\ndata rate clock cycles must be > 1 PMA RefClk cycle."]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - Reserved"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:15 - USB SuperSpeed TX LFPS Stretch : Minimum number of data rate \n\nclock cycles in which PMA tx_lfps_en signal is asserted. Number of \n\ndata rate clock cycles must be > 1 PMA RefClk cycle."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - USB SuperSpeed TX LFPS Stretch : Minimum number of data rate \n\nclock cycles in which PMA tx_lfps_en signal is asserted. Number of \n\ndata rate clock cycles must be > 1 PMA RefClk cycle."]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PipeCmnCtrl2Spec> {
        Field0W::new(self, 12)
    }
}
#[doc = "PIPE common control2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_cmn_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_cmn_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipeCmnCtrl2Spec;
impl crate::RegisterSpec for PipeCmnCtrl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipe_cmn_ctrl2::R`](R) reader structure"]
impl crate::Readable for PipeCmnCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pipe_cmn_ctrl2::W`](W) writer structure"]
impl crate::Writable for PipeCmnCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PIPE_CMN_CTRL2 to value 0xb000"]
impl crate::Resettable for PipeCmnCtrl2Spec {
    const RESET_VALUE: u16 = 0xb000;
}
