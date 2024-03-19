#[doc = "Register `DDR_DENALI_CTL_04` reader"]
pub type R = crate::R<DdrDenaliCtl04Spec>;
#[doc = "Register `DDR_DENALI_CTL_04` writer"]
pub type W = crate::W<DdrDenaliCtl04Spec>;
#[doc = "Field `DENALI0_WRCMD_SIDE_FIFO_LOG2_DEPTH` reader - Reports the depth of the DENALI port 0 processing FIFO. Value is the log2 value of the depth. READ- ONLY"]
pub type Denali0WrcmdSideFifoLog2DepthR = crate::FieldReader;
#[doc = "Field `DFS_CLOSE_BANKS` reader - Close all pages before doing DFS. Set to 1 to enable."]
pub type DfsCloseBanksR = crate::BitReader;
#[doc = "Field `DFS_CLOSE_BANKS` writer - Close all pages before doing DFS. Set to 1 to enable."]
pub type DfsCloseBanksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reports the depth of the DENALI port 0 processing FIFO. Value is the log2 value of the depth. READ- ONLY"]
    #[inline(always)]
    pub fn denali0_wrcmd_side_fifo_log2_depth(&self) -> Denali0WrcmdSideFifoLog2DepthR {
        Denali0WrcmdSideFifoLog2DepthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Close all pages before doing DFS. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_close_banks(&self) -> DfsCloseBanksR {
        DfsCloseBanksR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Close all pages before doing DFS. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_close_banks(&mut self) -> DfsCloseBanksW<DdrDenaliCtl04Spec> {
        DfsCloseBanksW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_04::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_04::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl04Spec;
impl crate::RegisterSpec for DdrDenaliCtl04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_04::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl04Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_04::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_04 to value 0"]
impl crate::Resettable for DdrDenaliCtl04Spec {
    const RESET_VALUE: u32 = 0;
}
