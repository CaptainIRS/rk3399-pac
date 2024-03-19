#[doc = "Register `CRU_SOFTRST_CON0` reader"]
pub type R = crate::R<CruSoftrstCon0Spec>;
#[doc = "Register `CRU_SOFTRST_CON0` writer"]
pub type W = crate::W<CruSoftrstCon0Spec>;
#[doc = "Field `CORE0_L_SRSTN_REQ` reader - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core0LSrstnReqR = crate::BitReader;
#[doc = "Field `CORE0_L_SRSTN_REQ` writer - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core0LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_B_SRSTN_REQ` reader - core0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core0BSrstnReqR = crate::BitReader;
#[doc = "Field `CORE0_B_SRSTN_REQ` writer - core0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Core0BSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO0_L_SRSTN_REQ` reader - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo0LSrstnReqR = crate::BitReader;
#[doc = "Field `COREPO0_L_SRSTN_REQ` writer - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo0LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO0_B_SRSTN_REQ` reader - corepo0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo0BSrstnReqR = crate::BitReader;
#[doc = "Field `COREPO0_B_SRSTN_REQ` writer - corepo0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type Corepo0BSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_L_SRSTN_REQ` reader - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type L2LSrstnReqR = crate::BitReader;
#[doc = "Field `L2_L_SRSTN_REQ` writer - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type L2LSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_B_SRSTN_REQ` reader - l2_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type L2BSrstnReqR = crate::BitReader;
#[doc = "Field `L2_B_SRSTN_REQ` writer - l2_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type L2BSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADB_L_SRSTN_REQ` reader - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type AdbLSrstnReqR = crate::BitReader;
#[doc = "Field `ADB_L_SRSTN_REQ` writer - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type AdbLSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADB_B_SRSTN_REQ` reader - adb_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type AdbBSrstnReqR = crate::BitReader;
#[doc = "Field `ADB_B_SRSTN_REQ` writer - adb_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
pub type AdbBSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_CCI_REQ` reader - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCciReqR = crate::BitReader;
#[doc = "Field `ARESETN_CCI_REQ` writer - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCciReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_CCIM0_NOC_REQ` reader - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCcim0NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_CCIM0_NOC_REQ` writer - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCcim0NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_CCIM1_NOC_REQ` reader - aresetn_ccim1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCcim1NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_CCIM1_NOC_REQ` writer - aresetn_ccim1_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnCcim1NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DBG_NOC_REQ` reader - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDbgNocReqR = crate::BitReader;
#[doc = "Field `RESETN_DBG_NOC_REQ` writer - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDbgNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core0_l_srstn_req(&self) -> Core0LSrstnReqR {
        Core0LSrstnReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - core0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core0_b_srstn_req(&self) -> Core0BSrstnReqR {
        Core0BSrstnReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo0_l_srstn_req(&self) -> Corepo0LSrstnReqR {
        Corepo0LSrstnReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - corepo0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo0_b_srstn_req(&self) -> Corepo0BSrstnReqR {
        Corepo0BSrstnReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn l2_l_srstn_req(&self) -> L2LSrstnReqR {
        L2LSrstnReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - l2_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn l2_b_srstn_req(&self) -> L2BSrstnReqR {
        L2BSrstnReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn adb_l_srstn_req(&self) -> AdbLSrstnReqR {
        AdbLSrstnReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - adb_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn adb_b_srstn_req(&self) -> AdbBSrstnReqR {
        AdbBSrstnReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_cci_req(&self) -> AresetnCciReqR {
        AresetnCciReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_ccim0_noc_req(&self) -> AresetnCcim0NocReqR {
        AresetnCcim0NocReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aresetn_ccim1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_ccim1_noc_req(&self) -> AresetnCcim1NocReqR {
        AresetnCcim1NocReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_dbg_noc_req(&self) -> ResetnDbgNocReqR {
        ResetnDbgNocReqR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core0_l_srstn_req(&mut self) -> Core0LSrstnReqW<CruSoftrstCon0Spec> {
        Core0LSrstnReqW::new(self, 0)
    }
    #[doc = "Bit 1 - core0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core0_b_srstn_req(&mut self) -> Core0BSrstnReqW<CruSoftrstCon0Spec> {
        Core0BSrstnReqW::new(self, 1)
    }
    #[doc = "Bit 2 - corepo0_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo0_l_srstn_req(&mut self) -> Corepo0LSrstnReqW<CruSoftrstCon0Spec> {
        Corepo0LSrstnReqW::new(self, 2)
    }
    #[doc = "Bit 3 - corepo0_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo0_b_srstn_req(&mut self) -> Corepo0BSrstnReqW<CruSoftrstCon0Spec> {
        Corepo0BSrstnReqW::new(self, 3)
    }
    #[doc = "Bit 4 - l2_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn l2_l_srstn_req(&mut self) -> L2LSrstnReqW<CruSoftrstCon0Spec> {
        L2LSrstnReqW::new(self, 4)
    }
    #[doc = "Bit 5 - l2_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn l2_b_srstn_req(&mut self) -> L2BSrstnReqW<CruSoftrstCon0Spec> {
        L2BSrstnReqW::new(self, 5)
    }
    #[doc = "Bit 6 - adb_l_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn adb_l_srstn_req(&mut self) -> AdbLSrstnReqW<CruSoftrstCon0Spec> {
        AdbLSrstnReqW::new(self, 6)
    }
    #[doc = "Bit 7 - adb_b_srstn request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn adb_b_srstn_req(&mut self) -> AdbBSrstnReqW<CruSoftrstCon0Spec> {
        AdbBSrstnReqW::new(self, 7)
    }
    #[doc = "Bit 8 - aresetn_cci request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_cci_req(&mut self) -> AresetnCciReqW<CruSoftrstCon0Spec> {
        AresetnCciReqW::new(self, 8)
    }
    #[doc = "Bit 9 - aresetn_ccim0_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_ccim0_noc_req(&mut self) -> AresetnCcim0NocReqW<CruSoftrstCon0Spec> {
        AresetnCcim0NocReqW::new(self, 9)
    }
    #[doc = "Bit 10 - aresetn_ccim1_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_ccim1_noc_req(&mut self) -> AresetnCcim1NocReqW<CruSoftrstCon0Spec> {
        AresetnCcim1NocReqW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_dbg_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_dbg_noc_req(&mut self) -> ResetnDbgNocReqW<CruSoftrstCon0Spec> {
        ResetnDbgNocReqW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon0Spec;
impl crate::RegisterSpec for CruSoftrstCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con0::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con0::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON0 to value 0"]
impl crate::Resettable for CruSoftrstCon0Spec {
    const RESET_VALUE: u32 = 0;
}
