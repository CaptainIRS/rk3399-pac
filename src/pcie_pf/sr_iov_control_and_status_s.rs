#[doc = "Register `SR_IOV_CONTROL_AND_STATUS_S` reader"]
pub type R = crate::R<SrIovControlAndStatusSSpec>;
#[doc = "Register `SR_IOV_CONTROL_AND_STATUS_S` writer"]
pub type W = crate::W<SrIovControlAndStatusSSpec>;
#[doc = "Field `VFE` reader - VF Enable \\[VFE\\]
This bit must be set to enable the VFs associated with this PF."]
pub type VfeR = crate::BitReader;
#[doc = "Field `VFE` writer - VF Enable \\[VFE\\]
This bit must be set to enable the VFs associated with this PF."]
pub type VfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFME` reader - VF Migration Enable \\[VFME\\]
Not supported. Hardwired to 0"]
pub type VfmeR = crate::BitReader;
#[doc = "Field `VFMIE` reader - VF Migration Interrupt Enable \\[VFMIE\\]
Not supported. Hardwired to 0"]
pub type VfmieR = crate::BitReader;
#[doc = "Field `VFMSE` reader - VF Memory Space Enable \\[VFMSE\\]
This bit must be set to allow access to the memory space of the VFs associated with this PF."]
pub type VfmseR = crate::BitReader;
#[doc = "Field `VFMSE` writer - VF Memory Space Enable \\[VFMSE\\]
This bit must be set to allow access to the memory space of the VFs associated with this PF."]
pub type VfmseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARIE` reader - ARI Capable Hierarchy \\[ARIE\\]
This bit enables the ARI mode for Virtual Functions. This bit must be set when VF Enable is set. Valid only for PF0"]
pub type ArieR = crate::BitReader;
#[doc = "Field `ARIE` writer - ARI Capable Hierarchy \\[ARIE\\]
This bit enables the ARI mode for Virtual Functions. This bit must be set when VF Enable is set. Valid only for PF0"]
pub type ArieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader<u16>;
#[doc = "Field `SSR` reader - SRIOV Status Register \\[SSR\\]
Not implemented."]
pub type SsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - VF Enable \\[VFE\\]
This bit must be set to enable the VFs associated with this PF."]
    #[inline(always)]
    pub fn vfe(&self) -> VfeR {
        VfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VF Migration Enable \\[VFME\\]
Not supported. Hardwired to 0"]
    #[inline(always)]
    pub fn vfme(&self) -> VfmeR {
        VfmeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VF Migration Interrupt Enable \\[VFMIE\\]
Not supported. Hardwired to 0"]
    #[inline(always)]
    pub fn vfmie(&self) -> VfmieR {
        VfmieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VF Memory Space Enable \\[VFMSE\\]
This bit must be set to allow access to the memory space of the VFs associated with this PF."]
    #[inline(always)]
    pub fn vfmse(&self) -> VfmseR {
        VfmseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARI Capable Hierarchy \\[ARIE\\]
This bit enables the ARI mode for Virtual Functions. This bit must be set when VF Enable is set. Valid only for PF0"]
    #[inline(always)]
    pub fn arie(&self) -> ArieR {
        ArieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:31 - SRIOV Status Register \\[SSR\\]
Not implemented."]
    #[inline(always)]
    pub fn ssr(&self) -> SsrR {
        SsrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - VF Enable \\[VFE\\]
This bit must be set to enable the VFs associated with this PF."]
    #[inline(always)]
    #[must_use]
    pub fn vfe(&mut self) -> VfeW<SrIovControlAndStatusSSpec> {
        VfeW::new(self, 0)
    }
    #[doc = "Bit 3 - VF Memory Space Enable \\[VFMSE\\]
This bit must be set to allow access to the memory space of the VFs associated with this PF."]
    #[inline(always)]
    #[must_use]
    pub fn vfmse(&mut self) -> VfmseW<SrIovControlAndStatusSSpec> {
        VfmseW::new(self, 3)
    }
    #[doc = "Bit 4 - ARI Capable Hierarchy \\[ARIE\\]
This bit enables the ARI mode for Virtual Functions. This bit must be set when VF Enable is set. Valid only for PF0"]
    #[inline(always)]
    #[must_use]
    pub fn arie(&mut self) -> ArieW<SrIovControlAndStatusSSpec> {
        ArieW::new(self, 4)
    }
}
#[doc = "SR-IOV Control and Status Registers Not implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_iov_control_and_status_s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_iov_control_and_status_s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrIovControlAndStatusSSpec;
impl crate::RegisterSpec for SrIovControlAndStatusSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_iov_control_and_status_s::R`](R) reader structure"]
impl crate::Readable for SrIovControlAndStatusSSpec {}
#[doc = "`write(|w| ..)` method takes [`sr_iov_control_and_status_s::W`](W) writer structure"]
impl crate::Writable for SrIovControlAndStatusSSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR_IOV_CONTROL_AND_STATUS_S to value 0"]
impl crate::Resettable for SrIovControlAndStatusSSpec {
    const RESET_VALUE: u32 = 0;
}
