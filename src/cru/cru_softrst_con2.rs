#[doc = "Register `CRU_SOFTRST_CON2` reader"]
pub type R = crate::R<CruSoftrstCon2Spec>;
#[doc = "Register `CRU_SOFTRST_CON2` writer"]
pub type W = crate::W<CruSoftrstCon2Spec>;
#[doc = "Field `CORE0_B_SRSTN_REQ_T` reader - core0_b_srstn request bit When HIGH, reset relative logic"]
pub type Core0BSrstnReqTR = crate::BitReader;
#[doc = "Field `CORE0_B_SRSTN_REQ_T` writer - core0_b_srstn request bit When HIGH, reset relative logic"]
pub type Core0BSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_B_SRSTN_REQ` reader - core1_b_srstn request bit When HIGH, reset relative logic"]
pub type Core1BSrstnReqR = crate::BitReader;
#[doc = "Field `CORE1_B_SRSTN_REQ` writer - core1_b_srstn request bit When HIGH, reset relative logic"]
pub type Core1BSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO0_B_SRSTN_REQ_T` reader - corepo0_b_srstn request bit When HIGH, reset relative logic"]
pub type Corepo0BSrstnReqTR = crate::BitReader;
#[doc = "Field `COREPO0_B_SRSTN_REQ_T` writer - corepo0_b_srstn request bit When HIGH, reset relative logic"]
pub type Corepo0BSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREPO1_B_SRSTN_REQ` reader - corepo1_b_srstn request bit When HIGH, reset relative logic"]
pub type Corepo1BSrstnReqR = crate::BitReader;
#[doc = "Field `COREPO1_B_SRSTN_REQ` writer - corepo1_b_srstn request bit When HIGH, reset relative logic"]
pub type Corepo1BSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARSTN_ADB400_GIC2COREB_REQ` reader - arstn_adb400_gic2coreb request bit When HIGH, reset relative logic"]
pub type ArstnAdb400Gic2corebReqR = crate::BitReader;
#[doc = "Field `ARSTN_ADB400_GIC2COREB_REQ` writer - arstn_adb400_gic2coreb request bit When HIGH, reset relative logic"]
pub type ArstnAdb400Gic2corebReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARSTN_ADB400_COREB2GIC_REQ` reader - arstn_adb400_coreb2gic request bit When HIGH, reset relative logic"]
pub type ArstnAdb400Coreb2gicReqR = crate::BitReader;
#[doc = "Field `ARSTN_ADB400_COREB2GIC_REQ` writer - arstn_adb400_coreb2gic request bit When HIGH, reset relative logic"]
pub type ArstnAdb400Coreb2gicReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTN_DBG_B_REQ` reader - prstn_dbg_b request bit When HIGH, reset relative logic"]
pub type PrstnDbgBReqR = crate::BitReader;
#[doc = "Field `PRSTN_DBG_B_REQ` writer - prstn_dbg_b request bit When HIGH, reset relative logic"]
pub type PrstnDbgBReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_B_SRSTN_REQ_T` reader - l2_b_srstn request bit When HIGH, reset relative logic"]
pub type L2BSrstnReqTR = crate::BitReader;
#[doc = "Field `L2_B_SRSTN_REQ_T` writer - l2_b_srstn request bit When HIGH, reset relative logic"]
pub type L2BSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADB_B_SRSTN_REQ_T` reader - adb_b_srstn request bit When HIGH, reset relative logic"]
pub type AdbBSrstnReqTR = crate::BitReader;
#[doc = "Field `ADB_B_SRSTN_REQ_T` writer - adb_b_srstn request bit When HIGH, reset relative logic"]
pub type AdbBSrstnReqTW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RKPERF_B_ARSTN_REQ` reader - rkperf_b_arstn request bit When HIGH, reset relative logic"]
pub type RkperfBArstnReqR = crate::BitReader;
#[doc = "Field `RKPERF_B_ARSTN_REQ` writer - rkperf_b_arstn request bit When HIGH, reset relative logic"]
pub type RkperfBArstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTM_CORE_B_SRSTN_REQ` reader - pvtm_core_b_srstn request bit When HIGH, reset relative logic"]
pub type PvtmCoreBSrstnReqR = crate::BitReader;
#[doc = "Field `PVTM_CORE_B_SRSTN_REQ` writer - pvtm_core_b_srstn request bit When HIGH, reset relative logic"]
pub type PvtmCoreBSrstnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - core0_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core0_b_srstn_req_t(&self) -> Core0BSrstnReqTR {
        Core0BSrstnReqTR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - core1_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn core1_b_srstn_req(&self) -> Core1BSrstnReqR {
        Core1BSrstnReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - corepo0_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo0_b_srstn_req_t(&self) -> Corepo0BSrstnReqTR {
        Corepo0BSrstnReqTR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - corepo1_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn corepo1_b_srstn_req(&self) -> Corepo1BSrstnReqR {
        Corepo1BSrstnReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - arstn_adb400_gic2coreb request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn arstn_adb400_gic2coreb_req(&self) -> ArstnAdb400Gic2corebReqR {
        ArstnAdb400Gic2corebReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - arstn_adb400_coreb2gic request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn arstn_adb400_coreb2gic_req(&self) -> ArstnAdb400Coreb2gicReqR {
        ArstnAdb400Coreb2gicReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - prstn_dbg_b request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn prstn_dbg_b_req(&self) -> PrstnDbgBReqR {
        PrstnDbgBReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - l2_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn l2_b_srstn_req_t(&self) -> L2BSrstnReqTR {
        L2BSrstnReqTR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - adb_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn adb_b_srstn_req_t(&self) -> AdbBSrstnReqTR {
        AdbBSrstnReqTR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - rkperf_b_arstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn rkperf_b_arstn_req(&self) -> RkperfBArstnReqR {
        RkperfBArstnReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pvtm_core_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn pvtm_core_b_srstn_req(&self) -> PvtmCoreBSrstnReqR {
        PvtmCoreBSrstnReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core0_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core0_b_srstn_req_t(&mut self) -> Core0BSrstnReqTW<CruSoftrstCon2Spec> {
        Core0BSrstnReqTW::new(self, 0)
    }
    #[doc = "Bit 1 - core1_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn core1_b_srstn_req(&mut self) -> Core1BSrstnReqW<CruSoftrstCon2Spec> {
        Core1BSrstnReqW::new(self, 1)
    }
    #[doc = "Bit 4 - corepo0_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo0_b_srstn_req_t(&mut self) -> Corepo0BSrstnReqTW<CruSoftrstCon2Spec> {
        Corepo0BSrstnReqTW::new(self, 4)
    }
    #[doc = "Bit 5 - corepo1_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn corepo1_b_srstn_req(&mut self) -> Corepo1BSrstnReqW<CruSoftrstCon2Spec> {
        Corepo1BSrstnReqW::new(self, 5)
    }
    #[doc = "Bit 8 - arstn_adb400_gic2coreb request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn arstn_adb400_gic2coreb_req(&mut self) -> ArstnAdb400Gic2corebReqW<CruSoftrstCon2Spec> {
        ArstnAdb400Gic2corebReqW::new(self, 8)
    }
    #[doc = "Bit 9 - arstn_adb400_coreb2gic request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn arstn_adb400_coreb2gic_req(&mut self) -> ArstnAdb400Coreb2gicReqW<CruSoftrstCon2Spec> {
        ArstnAdb400Coreb2gicReqW::new(self, 9)
    }
    #[doc = "Bit 10 - prstn_dbg_b request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn prstn_dbg_b_req(&mut self) -> PrstnDbgBReqW<CruSoftrstCon2Spec> {
        PrstnDbgBReqW::new(self, 10)
    }
    #[doc = "Bit 12 - l2_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn l2_b_srstn_req_t(&mut self) -> L2BSrstnReqTW<CruSoftrstCon2Spec> {
        L2BSrstnReqTW::new(self, 12)
    }
    #[doc = "Bit 13 - adb_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn adb_b_srstn_req_t(&mut self) -> AdbBSrstnReqTW<CruSoftrstCon2Spec> {
        AdbBSrstnReqTW::new(self, 13)
    }
    #[doc = "Bit 14 - rkperf_b_arstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn rkperf_b_arstn_req(&mut self) -> RkperfBArstnReqW<CruSoftrstCon2Spec> {
        RkperfBArstnReqW::new(self, 14)
    }
    #[doc = "Bit 15 - pvtm_core_b_srstn request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_srstn_req(&mut self) -> PvtmCoreBSrstnReqW<CruSoftrstCon2Spec> {
        PvtmCoreBSrstnReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon2Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon2Spec;
impl crate::RegisterSpec for CruSoftrstCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con2::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con2::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON2 to value 0"]
impl crate::Resettable for CruSoftrstCon2Spec {
    const RESET_VALUE: u32 = 0;
}
