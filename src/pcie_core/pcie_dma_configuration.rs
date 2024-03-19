#[doc = "Register `PCIE_DMA_CONFIGURATION` reader"]
pub type R = crate::R<PcieDmaConfigurationSpec>;
#[doc = "Field `NUM_CHANNELS` reader - num_channels\n\nNumber of uDMA Channels"]
pub type NumChannelsR = crate::FieldReader;
#[doc = "Field `NUM_PARTITIONS` reader - num_partitions\n\nNumber of DPRAM Partitions"]
pub type NumPartitionsR = crate::FieldReader;
#[doc = "Field `PARTITION_SIZE` reader - partition_size\n\nSize of each Partition"]
pub type PartitionSizeR = crate::FieldReader;
#[doc = "Field `SYS_AW_GT_32` reader - sys_aw_gt_32\n\nSys Addr Width > 32-bits"]
pub type SysAwGt32R = crate::BitReader;
#[doc = "Field `SYS_TW_GT_32` reader - sys_tw_gt_32\n\nSys Attr Width > 32-bits"]
pub type SysTwGt32R = crate::BitReader;
#[doc = "Field `EXT_AW_GT_32` reader - ext_aw_gt_32\n\nExt Addr Width > 32-bits"]
pub type ExtAwGt32R = crate::BitReader;
#[doc = "Field `EXT_TW_GT_32` reader - ext_tw_gt_32\n\nExt Attr Width > 32-bits"]
pub type ExtTwGt32R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - num_channels\n\nNumber of uDMA Channels"]
    #[inline(always)]
    pub fn num_channels(&self) -> NumChannelsR {
        NumChannelsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - num_partitions\n\nNumber of DPRAM Partitions"]
    #[inline(always)]
    pub fn num_partitions(&self) -> NumPartitionsR {
        NumPartitionsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - partition_size\n\nSize of each Partition"]
    #[inline(always)]
    pub fn partition_size(&self) -> PartitionSizeR {
        PartitionSizeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - sys_aw_gt_32\n\nSys Addr Width > 32-bits"]
    #[inline(always)]
    pub fn sys_aw_gt_32(&self) -> SysAwGt32R {
        SysAwGt32R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - sys_tw_gt_32\n\nSys Attr Width > 32-bits"]
    #[inline(always)]
    pub fn sys_tw_gt_32(&self) -> SysTwGt32R {
        SysTwGt32R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ext_aw_gt_32\n\nExt Addr Width > 32-bits"]
    #[inline(always)]
    pub fn ext_aw_gt_32(&self) -> ExtAwGt32R {
        ExtAwGt32R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ext_tw_gt_32\n\nExt Attr Width > 32-bits"]
    #[inline(always)]
    pub fn ext_tw_gt_32(&self) -> ExtTwGt32R {
        ExtTwGt32R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "PCIe DMA Configuration Register\n\nReserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_configuration::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaConfigurationSpec;
impl crate::RegisterSpec for PcieDmaConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_configuration::R`](R) reader structure"]
impl crate::Readable for PcieDmaConfigurationSpec {}
#[doc = "`reset()` method sets PCIE_DMA_CONFIGURATION to value 0xc522"]
impl crate::Resettable for PcieDmaConfigurationSpec {
    const RESET_VALUE: u32 = 0xc522;
}
