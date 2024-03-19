#[doc = "Register `PCIE_RC_ROOT_STATUS` reader"]
pub type R = crate::R<PcieRcRootStatusSpec>;
#[doc = "Register `PCIE_RC_ROOT_STATUS` writer"]
pub type W = crate::W<PcieRcRootStatusSpec>;
#[doc = "Field `PMERID` reader - PME Requester ID \\[PMERID\\]
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type PmeridR = crate::FieldReader<u16>;
#[doc = "Field `PMES` reader - PME Status \\[PMES\\]
This field is not set by the core but can be cleared by writing a 1 from the local management APB bus. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type PmesR = crate::BitReader;
#[doc = "Field `PMES` writer - PME Status \\[PMES\\]
This field is not set by the core but can be cleared by writing a 1 from the local management APB bus. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type PmesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PMEP` reader - PME Pending \\[PMEP\\]
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type PmepR = crate::BitReader;
#[doc = "Field `R18` reader - Reserved \\[R18\\]
Reserved"]
pub type R18R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PME Requester ID \\[PMERID\\]
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn pmerid(&self) -> PmeridR {
        PmeridR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - PME Status \\[PMES\\]
This field is not set by the core but can be cleared by writing a 1 from the local management APB bus. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn pmes(&self) -> PmesR {
        PmesR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PME Pending \\[PMEP\\]
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn pmep(&self) -> PmepR {
        PmepR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - Reserved \\[R18\\]
Reserved"]
    #[inline(always)]
    pub fn r18(&self) -> R18R {
        R18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - PME Status \\[PMES\\]
This field is not set by the core but can be cleared by writing a 1 from the local management APB bus. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn pmes(&mut self) -> PmesW<PcieRcRootStatusSpec> {
        PmesW::new(self, 16)
    }
}
#[doc = "Root Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcRootStatusSpec;
impl crate::RegisterSpec for PcieRcRootStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_root_status::R`](R) reader structure"]
impl crate::Readable for PcieRcRootStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_root_status::W`](W) writer structure"]
impl crate::Writable for PcieRcRootStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0000;
}
#[doc = "`reset()` method sets PCIE_RC_ROOT_STATUS to value 0"]
impl crate::Resettable for PcieRcRootStatusSpec {
    const RESET_VALUE: u32 = 0;
}
