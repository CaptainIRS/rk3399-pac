#[doc = "Register `DDR_DENALI_CTL_199` reader"]
pub type R = crate::R<DdrDenaliCtl199Spec>;
#[doc = "Register `DDR_DENALI_CTL_199` writer"]
pub type W = crate::W<DdrDenaliCtl199Spec>;
#[doc = "Field `Q_FULLNESS` reader - Quantity that determines command queue full."]
pub type QFullnessR = crate::FieldReader;
#[doc = "Field `Q_FULLNESS` writer - Quantity that determines command queue full."]
pub type QFullnessW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_ORDER_ACCEPT` reader - Forces the controller to accept commands in the order in which they are placed in the command queue."]
pub type InOrderAcceptR = crate::BitReader;
#[doc = "Field `IN_ORDER_ACCEPT` writer - Forces the controller to accept commands in the order in which they are placed in the command queue."]
pub type InOrderAcceptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:18 - Quantity that determines command queue full."]
    #[inline(always)]
    pub fn q_fullness(&self) -> QFullnessR {
        QFullnessR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Forces the controller to accept commands in the order in which they are placed in the command queue."]
    #[inline(always)]
    pub fn in_order_accept(&self) -> InOrderAcceptR {
        InOrderAcceptR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - Quantity that determines command queue full."]
    #[inline(always)]
    #[must_use]
    pub fn q_fullness(&mut self) -> QFullnessW<DdrDenaliCtl199Spec> {
        QFullnessW::new(self, 16)
    }
    #[doc = "Bit 24 - Forces the controller to accept commands in the order in which they are placed in the command queue."]
    #[inline(always)]
    #[must_use]
    pub fn in_order_accept(&mut self) -> InOrderAcceptW<DdrDenaliCtl199Spec> {
        InOrderAcceptW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_199::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_199::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl199Spec;
impl crate::RegisterSpec for DdrDenaliCtl199Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_199::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl199Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_199::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl199Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_199 to value 0"]
impl crate::Resettable for DdrDenaliCtl199Spec {
    const RESET_VALUE: u32 = 0;
}
