//! Mitra Priority Model & Arbitration Engine
//!
//! Mengatur arbitrase prioritas audio:
//! Fire Alarm (Emergency: 3) > Intercom / Paging (Normal: 2) > BGM (Low: 1).

use crate::error::PriorityError;
use crate::protocol::PriorityLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveStreamState {
    pub channel_id: u16,
    pub priority: PriorityLevel,
    pub session_id: u32,
}

pub struct PriorityArbitrator {
    active_streams: [Option<ActiveStreamState>; 64], // Hingga 64 concurrent channels
}

impl Default for PriorityArbitrator {
    fn default() -> Self {
        Self::new()
    }
}

impl PriorityArbitrator {
    pub fn new() -> Self {
        Self {
            active_streams: [None; 64],
        }
    }

    /// Meminta izin alokasi siaran pada channel tertentu.
    /// Jika channel sedang digunakan oleh prioritas yang lebih rendah, siaran lama dipotong (preemption).
    /// Jika channel sedang digunakan oleh prioritas yang lebih tinggi atau sama, permintaan ditolak.
    pub fn request_stream(
        &mut self,
        channel_id: u16,
        priority: PriorityLevel,
        session_id: u32,
    ) -> Result<Option<ActiveStreamState>, PriorityError> {
        let idx = (channel_id as usize) % self.active_streams.len();

        if let Some(active) = self.active_streams[idx] {
            if active.channel_id == channel_id {
                let active_val = active.priority as u8;
                let req_val = priority as u8;

                if req_val > active_val {
                    // Preemption: Prioritas lebih tinggi menginterupsi prioritas lebih rendah
                    let preempted = Some(active);
                    self.active_streams[idx] = Some(ActiveStreamState {
                        channel_id,
                        priority,
                        session_id,
                    });
                    return Ok(preempted);
                } else {
                    return Err(PriorityError::PreemptedByHigherPriority {
                        active: active_val,
                        requested: req_val,
                    });
                }
            }
        }

        self.active_streams[idx] = Some(ActiveStreamState {
            channel_id,
            priority,
            session_id,
        });

        Ok(None)
    }

    /// Melepaskan siaran aktif saat sesi selesai
    pub fn release_stream(&mut self, channel_id: u16, session_id: u32) -> bool {
        let idx = (channel_id as usize) % self.active_streams.len();

        if let Some(active) = self.active_streams[idx] {
            if active.channel_id == channel_id && active.session_id == session_id {
                self.active_streams[idx] = None;
                return true;
            }
        }
        false
    }

    pub fn get_active_priority(&self, channel_id: u16) -> Option<PriorityLevel> {
        let idx = (channel_id as usize) % self.active_streams.len();
        self.active_streams[idx].and_then(|st| {
            if st.channel_id == channel_id {
                Some(st.priority)
            } else {
                None
            }
        })
    }
}
