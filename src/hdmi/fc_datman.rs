#[doc = "Register `FC_DATMAN` writer"]
pub type W = crate::W<FcDatmanSpec>;
#[doc = "Field `ACP_TX` writer - ACP packet"]
pub type AcpTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR1_TX` writer - ISRC1 packet"]
pub type Iscr1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISCR2_TX` writer - ISRC2 packet"]
pub type Iscr2TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSD_TX` writer - VSD packet"]
pub type VsdTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPD_TX` writer - SPD packet"]
pub type SpdTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NULL_TX` writer - Null packet"]
pub type NullTxW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - ACP packet"]
    #[inline(always)]
    #[must_use]
    pub fn acp_tx(&mut self) -> AcpTxW<FcDatmanSpec> {
        AcpTxW::new(self, 0)
    }
    #[doc = "Bit 1 - ISRC1 packet"]
    #[inline(always)]
    #[must_use]
    pub fn iscr1_tx(&mut self) -> Iscr1TxW<FcDatmanSpec> {
        Iscr1TxW::new(self, 1)
    }
    #[doc = "Bit 2 - ISRC2 packet"]
    #[inline(always)]
    #[must_use]
    pub fn iscr2_tx(&mut self) -> Iscr2TxW<FcDatmanSpec> {
        Iscr2TxW::new(self, 2)
    }
    #[doc = "Bit 3 - VSD packet"]
    #[inline(always)]
    #[must_use]
    pub fn vsd_tx(&mut self) -> VsdTxW<FcDatmanSpec> {
        VsdTxW::new(self, 3)
    }
    #[doc = "Bit 4 - SPD packet"]
    #[inline(always)]
    #[must_use]
    pub fn spd_tx(&mut self) -> SpdTxW<FcDatmanSpec> {
        SpdTxW::new(self, 4)
    }
    #[doc = "Bit 5 - Null packet"]
    #[inline(always)]
    #[must_use]
    pub fn null_tx(&mut self) -> NullTxW<FcDatmanSpec> {
        NullTxW::new(self, 5)
    }
}
#[doc = "Frame Composer Data Island Manual Packet Request Register\n\nRequests to the Frame Composer the data island packet insertion for NULL, SPD, VSD,\n\nISRC2, ISRC1 and ACP packets when FC_DATAUTO0 bit is in manual mode for the packet\n\nrequested.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_datman::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDatmanSpec;
impl crate::RegisterSpec for FcDatmanSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`fc_datman::W`](W) writer structure"]
impl crate::Writable for FcDatmanSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DATMAN to value 0"]
impl crate::Resettable for FcDatmanSpec {
    const RESET_VALUE: u8 = 0;
}
