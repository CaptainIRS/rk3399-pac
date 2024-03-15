#[doc = "Register `GRF_PCIE_PERF_CON2` reader"]
pub type R = crate::R<GrfPciePerfCon2Spec>;
#[doc = "Register `GRF_PCIE_PERF_CON2` writer"]
pub type W = crate::W<GrfPciePerfCon2Spec>;
#[doc = "Field `PCIE_SW_AR_COUNT_ID` reader - When sw_ar_cnt_id_type=1, only count the id designated by sw_ar_count_id"]
pub type PcieSwArCountIdR = crate::FieldReader;
#[doc = "Field `PCIE_SW_AR_COUNT_ID` writer - When sw_ar_cnt_id_type=1, only count the id designated by sw_ar_count_id"]
pub type PcieSwArCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PCIE_SW_AW_COUNT_ID` reader - When sw_aw_cnt_id_type=1, only count the id designated by sw_aw_count_id"]
pub type PcieSwAwCountIdR = crate::FieldReader;
#[doc = "Field `PCIE_SW_AW_COUNT_ID` writer - When sw_aw_cnt_id_type=1, only count the id designated by sw_aw_count_id"]
pub type PcieSwAwCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - When sw_ar_cnt_id_type=1, only count the id designated by sw_ar_count_id"]
    #[inline(always)]
    pub fn pcie_sw_ar_count_id(&self) -> PcieSwArCountIdR {
        PcieSwArCountIdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - When sw_aw_cnt_id_type=1, only count the id designated by sw_aw_count_id"]
    #[inline(always)]
    pub fn pcie_sw_aw_count_id(&self) -> PcieSwAwCountIdR {
        PcieSwAwCountIdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - When sw_ar_cnt_id_type=1, only count the id designated by sw_ar_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_ar_count_id(&mut self) -> PcieSwArCountIdW<GrfPciePerfCon2Spec> {
        PcieSwArCountIdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - When sw_aw_cnt_id_type=1, only count the id designated by sw_aw_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_aw_count_id(&mut self) -> PcieSwAwCountIdW<GrfPciePerfCon2Spec> {
        PcieSwAwCountIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfPciePerfCon2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfPciePerfCon2Spec;
impl crate::RegisterSpec for GrfPciePerfCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_pcie_perf_con2::R`](R) reader structure"]
impl crate::Readable for GrfPciePerfCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_pcie_perf_con2::W`](W) writer structure"]
impl crate::Writable for GrfPciePerfCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_PCIE_PERF_CON2 to value 0"]
impl crate::Resettable for GrfPciePerfCon2Spec {
    const RESET_VALUE: u32 = 0;
}
