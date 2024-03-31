#[doc = "Register `DENALI_CTL_239` reader"]
pub type R = crate::R<DenaliCtl239Spec>;
#[doc = "Register `DENALI_CTL_239` writer"]
pub type W = crate::W<DenaliCtl239Spec>;
#[doc = "Field `RDLVL_GATE_AREF_EN` reader - Enables refreshes and other non- data commands to execute in the middle of gate training. Set to 1 to enable."]
pub type RdlvlGateArefEnR = crate::BitReader;
#[doc = "Field `RDLVL_GATE_AREF_EN` writer - Enables refreshes and other non- data commands to execute in the middle of gate training. Set to 1 to enable."]
pub type RdlvlGateArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_ROTATE` reader - Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
pub type RdlvlRotateR = crate::BitReader;
#[doc = "Field `RDLVL_ROTATE` writer - Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
pub type RdlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_GATE_ROTATE` reader - Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
pub type RdlvlGateRotateR = crate::BitReader;
#[doc = "Field `RDLVL_GATE_ROTATE` writer - Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
pub type RdlvlGateRotateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables refreshes and other non- data commands to execute in the middle of gate training. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_gate_aref_en(&self) -> RdlvlGateArefEnR {
        RdlvlGateArefEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn rdlvl_rotate(&self) -> RdlvlRotateR {
        RdlvlRotateR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn rdlvl_gate_rotate(&self) -> RdlvlGateRotateR {
        RdlvlGateRotateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables refreshes and other non- data commands to execute in the middle of gate training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_aref_en(&mut self) -> RdlvlGateArefEnW<DenaliCtl239Spec> {
        RdlvlGateArefEnW::new(self, 0)
    }
    #[doc = "Bit 16 - Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_rotate(&mut self) -> RdlvlRotateW<DenaliCtl239Spec> {
        RdlvlRotateW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_rotate(&mut self) -> RdlvlGateRotateW<DenaliCtl239Spec> {
        RdlvlGateRotateW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_239::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_239::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl239Spec;
impl crate::RegisterSpec for DenaliCtl239Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_239::R`](R) reader structure"]
impl crate::Readable for DenaliCtl239Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_239::W`](W) writer structure"]
impl crate::Writable for DenaliCtl239Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_239 to value 0"]
impl crate::Resettable for DenaliCtl239Spec {
    const RESET_VALUE: u32 = 0;
}
